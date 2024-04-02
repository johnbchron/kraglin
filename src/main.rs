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
/// This represents every non-error type that can be sent, received, or used as
/// a key's value.
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

/// All commands supported by [`kraglin`](crate).
#[derive(Debug, Clone, PartialEq, Hash)]
pub enum Command {
  /// `SET`: Sets a key.
  Set { key: SmolStr, value: Value },
  /// `GET`: Gets a key.
  Get { key: SmolStr },
  /// `MGET`: Gets multiple keys.
  MultipleGet { keys: Vec<SmolStr> },
  /// `INCR`: Increments a key.
  ///
  /// This works for anything that looks like an integer.
  Increment { key: SmolStr },
  /// `KEYS`: Lists all keys.
  Keys,
  /// `EXISTS`: Checks whether a key exists.
  Exists { key: SmolStr },
  /// `DELETE`: Deletes a key.
  Delete { key: SmolStr },
  /// `INFO`: Returns server info.
  Info,
  /// `HSET`: Sets a field in a hash map.
  HashSet {
    key:   SmolStr,
    field: SmolStr,
    value: Value,
  },
  /// `HGET`: Gets the value of a hash map field.
  HashGet { key: SmolStr, field: SmolStr },
  /// `HGETALL`: Gets all the fields and values in a hash map.
  HashGetAll { key: SmolStr },
  /// `HMGET`: Gets multiple fields from a hash map.
  HashMultipleGet {
    key:    SmolStr,
    fields: Vec<SmolStr>,
  },
  /// `SADD`: Adds a value to a set.
  SetAdd { key: SmolStr, value: Value },
  /// `SMEMBERS`: Gets all the members of a set.
  SetMembers { key: SmolStr },
  /// `SCARD`: Gets the cardinality of a set.
  SetCardinality { key: SmolStr },
  /// `SISMEMBER`: Checks if a value is a member of a set.
  SetIsMember { key: SmolStr, value: Value },
  /// `SDIFF`: Returns the difference between two sets.
  SetDifference { set_a: SmolStr, set_b: SmolStr },
  /// `SDIFFSTORE`: Calculates and stores the difference between two sets.
  SetDifferenceStore {
    set_a:   SmolStr,
    set_b:   SmolStr,
    new_set: SmolStr,
  },
  /// `SREM`: Removes a value from a set.
  SetRemainder { key: SmolStr, value: Value },
  /// `LPUSH`: Pushes a value to a list head.
  LeftPush { key: SmolStr, value: Value },
  /// `RPUSH`: Pushes a value to a list tail.
  RightPush { key: SmolStr, value: Value },
  /// `LRANGE`: Returns values from a range within a list.
  ListRange {
    key:   SmolStr,
    start: i64,
    end:   i64,
  },
  /// `LLEN`: Returns the length of a list.
  ListLength { key: SmolStr },
  /// `LPOP`: Pops a value from a list head.
  LeftPop { key: SmolStr },
  /// `RPOP`: Pops a value from a list tail.
  RightPop { key: SmolStr },
}

/// The conglomerate error type for all [`kraglin`](crate) commands.
#[derive(Debug, Clone, thiserror::Error)]
pub enum KraglinError {}

/// A convenience type alias for a oneshot receiver with a [`kraglin`](crate)
/// result.
type ResultReceiver = oneshot::Receiver<Result<Value, KraglinError>>;

/// The generalized backend trait. All storage/execution backends implement
/// this.
pub trait Backend {
  fn execute(
    &self,
    command: Command,
    result_channel: ResultReceiver,
  ) -> impl Future<Output = ()> + Send;
}

#[tokio::main]
async fn main() {
  println!("Hello, world!");
}
