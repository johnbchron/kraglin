//! The naive `Backend` implementation, using a `Arc<Mutex<HashMap<SmolStr,
//! StoredValue>>>`.

use std::{
  collections::{BTreeMap, HashMap},
  hash::Hash,
  sync::Arc,
};

use smol_str::SmolStr;
use tokio::sync::Mutex;

use crate::{
  backends::Backend,
  command::Command,
  value::{StoredValue, Value},
  KraglinError,
};

/// A trait to extend `HashMap` to allow directly setting a key with `Option<V>`
trait SettableHashMap<K: Eq + Hash, V: Hash> {
  /// Sets a key with an optional value. If `val` is `Some()`, inserts the
  /// value. If `None`, deletes the previous value if it existed.
  fn set(&mut self, key: K, val: Option<V>);
}

impl<K: Eq + Hash, V: Hash> SettableHashMap<K, V> for HashMap<K, V> {
  fn set(&mut self, key: K, val: Option<V>) {
    match val {
      Some(v) => {
        self.insert(key, v);
      }
      None => {
        self.remove(&key);
      }
    }
  }
}

/// The naive `Backend` implementation, using a `Arc<Mutex<HashMap<SmolStr,
/// StoredValue>>>`.
pub struct SimpleBackend(Arc<Mutex<HashMap<SmolStr, StoredValue>>>);

impl Backend for SimpleBackend {
  fn new() -> SimpleBackend {
    SimpleBackend(Arc::new(Mutex::new(HashMap::new())))
  }

  async fn execute(&self, command: Command) -> Result<Value, KraglinError> {
    match command {
      Command::Set { key, value } => {
        let mut m = self.0.lock().await;
        m.set(key, value.into());
        Ok(Value::Nothing)
      }
      Command::Get { key } => {
        let m = self.0.lock().await;
        Ok(m.get(&key).cloned().into())
      }
      Command::MultipleGet { keys } => {
        let m = self.0.lock().await;
        let values = keys
          .into_iter()
          .map(|k| m.get(&k).cloned().into())
          .collect::<Vec<_>>();
        Ok(Value::Array(values))
      }
      Command::Increment { key } => {
        let mut m = self.0.lock().await;
        let entry = m.entry(key).or_insert(StoredValue::Integer(0));

        // try to parse the value as an `i64`, increment it, and then return the
        // incremented value as an Integer
        match entry {
          StoredValue::Integer(i) => {
            *i += 1;
            Ok(Value::Integer(*i))
          }
          StoredValue::BigNumber(n) => {
            let Ok(as_i64) = i64::try_from(n.clone()) else {
              return Err(KraglinError::OutOfRange);
            };
            *n = (as_i64 + 1).into();
            Ok(Value::Integer(as_i64 + 1))
          }
          StoredValue::SimpleString(s) => {
            if let Ok(as_i64) = s.parse::<i64>() {
              *s = format!("{}", as_i64 + 1).into();
              Ok(Value::Integer(as_i64 + 1))
            } else {
              Err(KraglinError::CannotParseAsInteger)
            }
          }
          StoredValue::BulkString(b) => {
            let Some(as_ascii) = b.as_ascii() else {
              return Err(KraglinError::CannotParseAsInteger);
            };
            let Ok(as_i64) = as_ascii.as_str().parse::<i64>() else {
              return Err(KraglinError::CannotParseAsInteger);
            };

            *b = format!("{}", as_i64 + 1).into();
            Ok(Value::Integer(as_i64 + 1))
          }
          _ => Err(KraglinError::WrongType),
        }
      }
      Command::Keys => {
        let m = self.0.lock().await;
        let mut keys = m.keys().cloned().collect::<Vec<_>>();
        keys.sort_unstable();
        Ok(Value::Array(
          keys.into_iter().map(Value::SimpleString).collect(),
        ))
      }
      Command::Exists { key } => {
        let m = self.0.lock().await;
        let exists = m.get(&key).is_some();
        Ok(Value::Integer(exists.into()))
      }
      Command::Delete { key } => {
        let mut m = self.0.lock().await;
        Ok(Value::Integer(m.remove(&key).is_some().into()))
      }
      Command::Info => {
        let m = self.0.lock().await;
        let key_count = m.keys().count();
        Ok(Value::SimpleString(
          format!(
            "We've got {key_count} key{} right now, thanks for asking :)",
            if key_count != 1 { "s" } else { "" }
          )
          .into(),
        ))
      }
      Command::HashSet { key, field, value } => {
        let mut m = self.0.lock().await;

        // get or insert, with a special case for `Nothing`
        let entry = m.entry(key).or_insert(StoredValue::Map(BTreeMap::new()));

        match entry {
          StoredValue::Map(m) => {
            let inserted = !m.contains_key(&field);
            m.insert(field, value);
            Ok(Value::Integer(inserted.into()))
          }
          _ => Err(KraglinError::WrongType),
        }
      }
      Command::HashGet { key, field } => {
        let m = self.0.lock().await;
        match m.get(&key) {
          Some(StoredValue::Map(h)) => match h.get(&field) {
            Some(v) => Ok(v.clone()),
            None => Ok(Value::Nothing),
          },
          Some(_) => Err(KraglinError::WrongType),
          None => Ok(Value::Nothing),
        }
      }
      Command::HashGetAll { key } => {
        let m = self.0.lock().await;
        match m.get(&key) {
          Some(StoredValue::Map(h)) => Ok(Value::Map(h.clone())),
          Some(_) => Err(KraglinError::WrongType),
          None => Ok(Value::Nothing),
        }
      }
      Command::HashMultipleGet { key, fields } => {
        let m = self.0.lock().await;

        let all_nothing = || {
          Ok(Value::Array(
            (0..fields.len()).map(|_| Value::Nothing).collect(),
          ))
        };

        match m.get(&key) {
          Some(StoredValue::Map(m)) => Ok(Value::Array(
            fields
              .into_iter()
              .map(|f| m.get(&f).cloned().unwrap_or(Value::Nothing))
              .collect(),
          )),
          Some(_) => Err(KraglinError::WrongType),
          None => all_nothing(),
        }
      }
      Command::SetAdd { key, value } => {
        let mut m = self.0.lock().await;

        let entry =
          m.entry(key).or_insert(StoredValue::Set(Default::default()));

        let set = match entry {
          StoredValue::Set(s) => s,
          _ => {
            return Err(KraglinError::WrongType);
          }
        };

        Ok(Value::Integer(set.insert(value) as i64))
      }
      Command::SetMembers { key } => {
        let m = self.0.lock().await;

        match m.get(&key) {
          Some(StoredValue::Set(s)) => Ok(Value::Set(s.clone())),
          Some(_) => Err(KraglinError::WrongType),
          None => Ok(Value::Set(Default::default())),
        }
      }
      Command::SetCardinality { key } => {
        let m = self.0.lock().await;

        match m.get(&key) {
          Some(StoredValue::Set(s)) => Ok(Value::Integer(s.len() as _)),
          Some(_) => Err(KraglinError::WrongType),
          None => Ok(Value::Integer(0)),
        }
      }
      Command::SetIsMember { key, value } => {
        let m = self.0.lock().await;

        match m.get(&key) {
          Some(StoredValue::Set(s)) => {
            Ok(Value::Integer(s.contains(&value) as _))
          }
          Some(_) => Err(KraglinError::WrongType),
          None => Ok(Value::Integer(0)),
        }
      }
      Command::SetDifference { set_a, set_b } => {
        let m = self.0.lock().await;

        match (m.get(&set_a), m.get(&set_b)) {
          // if both values exist and are sets
          (Some(StoredValue::Set(s1)), Some(StoredValue::Set(s2))) => {
            Ok(Value::Set(s1.difference(s2).cloned().collect()))
          }
          // if only the first one exists and is a set
          (Some(StoredValue::Set(s)), None) => Ok(Value::Set(s.clone())),
          // if only the second one exists and is a set
          (None, Some(StoredValue::Set(_))) => {
            Ok(Value::Set(Default::default()))
          }
          // if neither exist
          (None, None) => Ok(Value::Set(Default::default())),
          // under any other case
          _ => Err(KraglinError::WrongType),
        }
      }
      Command::SetDifferenceStore {
        set_a,
        set_b,
        new_set,
      } => {
        let mut m = self.0.lock().await;

        // this is the same logic as for SetDifference
        let new_set_value = match (m.get(&set_a), m.get(&set_b)) {
          (Some(StoredValue::Set(s1)), Some(StoredValue::Set(s2))) => {
            s1.difference(s2).cloned().collect()
          }
          (Some(StoredValue::Set(s)), None) => s.clone(),
          (None, Some(StoredValue::Set(_))) => Default::default(),
          (None, None) => Default::default(),
          _ => {
            return Err(KraglinError::WrongType);
          }
        };
        let new_set_size = new_set_value.len();

        m.insert(new_set, StoredValue::Set(new_set_value));

        Ok(Value::Integer(new_set_size as _))
      }
      Command::SetRemove { key, value } => {
        let mut m = self.0.lock().await;

        match m.get_mut(&key) {
          Some(StoredValue::Set(s)) => {
            Ok(Value::Integer(s.remove(&value) as _))
          }
          Some(_) => Err(KraglinError::WrongType),
          None => Ok(Value::Integer(0)),
        }
      }
      Command::LeftPush { key, value } => {
        let mut m = self.0.lock().await;

        match m.get_mut(&key) {
          Some(StoredValue::Array(a)) => {
            a.insert(0, value);
            Ok(Value::Integer(a.len() as _))
          }
          Some(_) => Err(KraglinError::WrongType),
          None => {
            m.insert(key, StoredValue::Array(vec![value]));
            Ok(Value::Integer(1))
          }
        }
      }
      Command::RightPush { key: _, value: _ } => todo!(),
      Command::ListRange {
        key: _,
        start: _,
        end: _,
      } => todo!(),
      Command::ListLength { key: _ } => todo!(),
      Command::LeftPop { key: _ } => todo!(),
      Command::RightPop { key: _ } => todo!(),
    }
  }
}
