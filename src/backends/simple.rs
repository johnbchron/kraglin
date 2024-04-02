use std::{
  collections::{BTreeMap, HashMap},
  sync::Arc,
};

use smol_str::SmolStr;
use tokio::sync::Mutex;

use crate::{backends::Backend, command::Command, value::Value, KraglinError};

pub struct SimpleBackend(Arc<Mutex<HashMap<SmolStr, Value>>>);

impl Backend for SimpleBackend {
  async fn execute(&self, command: Command) -> Result<Value, KraglinError> {
    match command {
      Command::Set { key, value } => {
        let mut m = self.0.lock().await;
        m.insert(key, value);
        Ok(Value::Nothing)
      }
      Command::Get { key } => {
        let m = self.0.lock().await;
        Ok(m.get(&key).cloned().unwrap_or(Value::Nothing))
      }
      Command::MultipleGet { keys } => {
        let m = self.0.lock().await;
        let values = keys
          .into_iter()
          .map(|k| m.get(&k).cloned().unwrap_or(Value::Nothing))
          .collect::<Vec<_>>();
        Ok(Value::Array(values))
      }
      Command::Increment { key } => {
        let mut m = self.0.lock().await;
        let entry = m.entry(key).or_insert(Value::Integer(0));
        if matches!(entry, Value::Nothing) {
          *entry = Value::Integer(0);
        }

        // try to parse the value as an `i64`, increment it, and then return the
        // incremented value as an Integer
        match entry {
          Value::Integer(i) => {
            *i += 1;
            Ok(Value::Integer(*i))
          }
          Value::BigNumber(n) => {
            let Ok(as_i64) = i64::try_from(n.clone()) else {
              return Err(KraglinError::OutOfRange);
            };
            *n = (as_i64 + 1).into();
            Ok(Value::Integer(as_i64 + 1))
          }
          Value::SimpleString(s) => {
            if let Ok(as_i64) = s.parse::<i64>() {
              *s = format!("{}", as_i64 + 1).into();
              Ok(Value::Integer(as_i64 + 1))
            } else {
              Err(KraglinError::CannotParseAsInteger)
            }
          }
          Value::BulkString(b) => {
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
        let keys = m
          .keys()
          .map(|k| Value::SimpleString(k.to_owned()))
          .collect::<Vec<_>>();
        Ok(Value::Array(keys))
      }
      Command::Exists { key } => {
        let m = self.0.lock().await;
        let exists = match m.get(&key) {
          Some(Value::Nothing) => false,
          Some(_) => true,
          None => false,
        };
        Ok(Value::Integer(exists.into()))
      }
      Command::Delete { key } => {
        let mut m = self.0.lock().await;
        Ok(Value::Integer(m.remove(&key).is_some().into()))
      }
      Command::Info => {
        let m = self.0.lock().await;
        Ok(Value::SimpleString(
          format!(
            "We've got {} keys right now, thanks for asking :)",
            m.keys().count()
          )
          .into(),
        ))
      }
      Command::HashSet { key, field, value } => {
        let mut m = self.0.lock().await;

        // get or insert, with a special case for `Nothing`
        let entry = m.entry(key).or_insert(Value::Map(BTreeMap::new()));
        if matches!(entry, Value::Nothing) {
          *entry = Value::Map(BTreeMap::new());
        }

        match entry {
          Value::Map(m) => {
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
          Some(Value::Map(h)) => match h.get(&field) {
            Some(v) => Ok(v.clone()),
            None => Ok(Value::Nothing),
          },
          Some(Value::Nothing) => Ok(Value::Nothing),
          Some(_) => Err(KraglinError::WrongType),
          None => Ok(Value::Nothing),
        }
      }
      Command::HashGetAll { key } => {
        let m = self.0.lock().await;
        match m.get(&key) {
          Some(Value::Map(h)) => Ok(Value::Map(h.clone())),
          Some(Value::Nothing) => Ok(Value::Nothing),
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
          Some(Value::Map(m)) => Ok(Value::Array(
            fields
              .into_iter()
              .map(|f| m.get(&f).cloned().unwrap_or(Value::Nothing))
              .collect(),
          )),
          Some(Value::Nothing) => all_nothing(),
          Some(_) => Err(KraglinError::WrongType),
          None => all_nothing(),
        }
      }
      Command::SetAdd { key: _, value: _ } => todo!(),
      Command::SetMembers { key: _ } => todo!(),
      Command::SetCardinality { key: _ } => todo!(),
      Command::SetIsMember { key: _, value: _ } => todo!(),
      Command::SetDifference { set_a: _, set_b: _ } => todo!(),
      Command::SetDifferenceStore {
        set_a: _,
        set_b: _,
        new_set: _,
      } => todo!(),
      Command::SetRemainder { key: _, value: _ } => todo!(),
      Command::LeftPush { key: _, value: _ } => todo!(),
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
