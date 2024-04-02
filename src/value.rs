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
