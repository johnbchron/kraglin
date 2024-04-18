//! Defines the `Command` item.

use smol_str::SmolStr;

use crate::value::Value;

/// All commands supported by [`kraglin`](crate).
#[derive(Debug, Clone, PartialEq, Hash)]
pub enum Command {
  /// `SET`: Sets a key.
  Set {
    /// The key to set.
    key:   SmolStr,
    /// The value to set the key with.
    value: Value,
  },
  /// `GET`: Gets a key.
  Get {
    /// The key to get.
    key: SmolStr,
  },
  /// `MGET`: Gets multiple keys.
  MultipleGet {
    /// The keys to get.
    keys: Vec<SmolStr>,
  },
  /// `INCR`: Increments a key.
  ///
  /// This works for anything that looks like an integer.
  Increment {
    /// The key to increment.
    ///
    /// This key is allowed to be any type that can be interpreted as a
    /// [`Value::Integer`], and will be incremented in place. The type will not
    /// change.
    key: SmolStr,
  },
  /// `KEYS`: Lists all keys.
  Keys,
  /// `EXISTS`: Checks whether a key exists.
  Exists {
    /// The key to check.
    key: SmolStr,
  },
  /// `DELETE`: Deletes a key.
  Delete {
    /// The key to delete.
    key: SmolStr,
  },
  /// `INFO`: Returns server info.
  Info,
  /// `HSET`: Sets a field in a hash map.
  HashSet {
    /// The (hash) key which contains the field to set.
    key:   SmolStr,
    /// The field to set.
    field: SmolStr,
    /// The value to set the field with.
    value: Value,
  },
  /// `HGET`: Gets the value of a hash map field.
  HashGet {
    /// The (hash) key which contains the field to get.
    key:   SmolStr,
    /// The field to get.
    field: SmolStr,
  },
  /// `HGETALL`: Gets all the fields and values in a hash map.
  HashGetAll {
    /// The (hash) key from which to get the fields and values.
    key: SmolStr,
  },
  /// `HMGET`: Gets multiple fields from a hash map.
  HashMultipleGet {
    /// The (hash) key which contains the fields to get.
    key:    SmolStr,
    /// The fields to get.
    fields: Vec<SmolStr>,
  },
  /// `SADD`: Adds a value to a set.
  SetAdd {
    /// The (set) key to which to add the value.
    key:   SmolStr,
    /// The value to add.
    value: Value,
  },
  /// `SMEMBERS`: Gets all the members of a set.
  SetMembers {
    /// The (set) key to get the set values of.
    key: SmolStr,
  },
  /// `SCARD`: Gets the cardinality of a set.
  SetCardinality {
    /// The (set) key to get the set cardinality of.
    key: SmolStr,
  },
  /// `SISMEMBER`: Checks if a value is a member of a set.
  SetIsMember {
    /// The (set) key to check for membership.
    key:   SmolStr,
    /// The value to check whether it is a member.
    value: Value,
  },
  /// `SDIFF`: Returns the difference between two sets.
  SetDifference {
    /// The key of the set to be subtracted against.
    set_a: SmolStr,
    /// The key of the set to subtract with.
    set_b: SmolStr,
  },
  /// `SDIFFSTORE`: Calculates and stores the difference between two sets.
  SetDifferenceStore {
    /// The key of the set to be subtracted against.
    set_a:   SmolStr,
    /// The key of the set to subtract with.
    set_b:   SmolStr,
    /// The key at which to store the difference.
    new_set: SmolStr,
  },
  /// `SREM`: Removes a value from a set.
  SetRemove {
    /// The (set) key to remove from.
    key:   SmolStr,
    /// The value to remove.
    value: Value,
  },
  /// `LPUSH`: Pushes a value to a list head.
  LeftPush {
    /// The (list) key to left-push to.
    key:   SmolStr,
    /// The value to left-push.
    value: Value,
  },
  /// `RPUSH`: Pushes a value to a list tail.
  RightPush {
    /// The (list) key to right-push to.
    key:   SmolStr,
    /// The value to right-push.
    value: Value,
  },
  /// `LRANGE`: Returns values from a range within a list.
  ListRange {
    /// The (list) key to pull a range from.
    key:   SmolStr,
    /// The start index of the range to pull.
    start: i64,
    /// The end index of the range to pull.
    end:   i64,
  },
  /// `LLEN`: Returns the length of a list.
  ListLength {
    /// The (list) key to check for length.
    key: SmolStr,
  },
  /// `LPOP`: Pops a value from a list head.
  LeftPop {
    /// The (list) key to left-pop from.
    key: SmolStr,
  },
  /// `RPOP`: Pops a value from a list tail.
  RightPop {
    /// The (list) key to right-pop from.
    key: SmolStr,
  },
}

impl Command {
  /// The RESP3 name of the command.
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
