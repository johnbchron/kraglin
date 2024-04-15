use smol_str::SmolStr;

use crate::value::Value;

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
  SetRemove { key: SmolStr, value: Value },
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

impl Command {
  pub fn command_name(&self) -> &'static str {
    match self {
      Command::Set { .. } => "SET",
      Command::Get { .. } => "GET",
      Command::MultipleGet { .. } => "MGET",
      Command::Increment { .. } => "INCR",
      Command::Keys => "KEYS",
      Command::Exists { .. } => "EXISTS",
      Command::Delete { .. } => "DEL",
      Command::Info => "INFO",
      Command::HashSet { .. } => "HSET",
      Command::HashGet { .. } => "HGET",
      Command::HashGetAll { .. } => "HGETALL",
      Command::HashMultipleGet { .. } => "HMGET",
      Command::SetAdd { .. } => "SADD",
      Command::SetMembers { .. } => "SMEMBERS",
      Command::SetCardinality { .. } => "SCARD",
      Command::SetIsMember { .. } => "SISMEMBER",
      Command::SetDifference { .. } => "SDIFF",
      Command::SetDifferenceStore { .. } => "SDIFFSTORE",
      Command::SetRemove { .. } => "SREM",
      Command::LeftPush { .. } => "LPUSH",
      Command::RightPush { .. } => "RPUSH",
      Command::ListRange { .. } => "LRANGE",
      Command::ListLength { .. } => "LLEN",
      Command::LeftPop { .. } => "LPOP",
      Command::RightPop { .. } => "RPOP",
    }
  }
}
