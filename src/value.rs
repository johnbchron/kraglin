use std::{
  collections::{BTreeMap, BTreeSet},
  hash::Hasher,
};

use educe::Educe;
use smol_str::SmolStr;

fn f64_hash<H: Hasher>(s: &f64, state: &mut H) {
  decorum::hash::FloatHash::float_hash(s, state);
}

/// The base value type in [`kraglin`](crate).
///
/// This represents every non-error type that can be sent, received, or used
/// as a key's value.
#[derive(Debug, Clone, PartialEq, Educe)]
#[educe(Hash)]
pub enum Value {
  /// A simple string. A simple string is not allowed to contain carraige
  /// return (`\r`) or line feed (`\n`) characters.
  SimpleString(SmolStr),
  /// An integer described by an [`i64`].
  Integer(i64),
  /// A string of arbitrary length containing arbitrary bytes.
  BulkString(bytes::Bytes),
  /// An array of any number of [`Value`] types.
  Array(Vec<Value>),
  /// A boolean value.
  Boolean(bool),
  /// A double precision float ([`f64`]).
  Double(#[educe(Hash(method(f64_hash)))] f64),
  /// A number which allows for values larger than an [`i64`].
  BigNumber(dashu_int::IBig),
  /// A string dictionary of [`Value`]s.
  Map(BTreeMap<SmolStr, Value>),
  /// A set of [`Value`]s. Follows the set definition of [`BTreeSet`].
  Set(BTreeSet<Value>),
  /// An unset value.
  Nothing,
}

/// The stored version of [`Value`]. The main difference is the absence of
/// `Nothing`.
#[derive(Debug, Clone, PartialEq, Educe)]
#[educe(Hash)]
pub enum StoredValue {
  /// A simple string. A simple string is not allowed to contain carraige
  /// return (`\r`) or line feed (`\n`) characters.
  SimpleString(SmolStr),
  /// An integer described by an [`i64`].
  Integer(i64),
  /// A string of arbitrary length containing arbitrary bytes.
  BulkString(bytes::Bytes),
  /// An array of any number of [`Value`] types.
  Array(Vec<Value>),
  /// A boolean value.
  Boolean(bool),
  /// A double precision float ([`f64`]).
  Double(#[educe(Hash(method(f64_hash)))] f64),
  /// A number which allows for values larger than an [`i64`].
  BigNumber(dashu_int::IBig),
  /// A string dictionary of [`Value`]s.
  Map(BTreeMap<SmolStr, Value>),
  /// A set of [`Value`]s. Follows the set definition of [`BTreeSet`].
  Set(BTreeSet<Value>),
}

impl From<Value> for Option<StoredValue> {
  fn from(value: Value) -> Self {
    match value {
      Value::SimpleString(s) => Some(StoredValue::SimpleString(s)),
      Value::Integer(i) => Some(StoredValue::Integer(i)),
      Value::BulkString(bs) => Some(StoredValue::BulkString(bs)),
      Value::Array(a) => Some(StoredValue::Array(a)),
      Value::Boolean(b) => Some(StoredValue::Boolean(b)),
      Value::Double(d) => Some(StoredValue::Double(d)),
      Value::BigNumber(bn) => Some(StoredValue::BigNumber(bn)),
      Value::Map(m) => Some(StoredValue::Map(m)),
      Value::Set(s) => Some(StoredValue::Set(s)),
      Value::Nothing => None,
    }
  }
}

impl From<Option<StoredValue>> for Value {
  fn from(value: Option<StoredValue>) -> Self {
    match value {
      Some(StoredValue::SimpleString(s)) => Value::SimpleString(s),
      Some(StoredValue::Integer(i)) => Value::Integer(i),
      Some(StoredValue::BulkString(bs)) => Value::BulkString(bs),
      Some(StoredValue::Array(a)) => Value::Array(a),
      Some(StoredValue::Boolean(b)) => Value::Boolean(b),
      Some(StoredValue::Double(d)) => Value::Double(d),
      Some(StoredValue::BigNumber(bn)) => Value::BigNumber(bn),
      Some(StoredValue::Map(m)) => Value::Map(m),
      Some(StoredValue::Set(s)) => Value::Set(s),
      None => todo!(),
    }
  }
}
