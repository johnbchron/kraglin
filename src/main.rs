#![feature(ascii_char)]

mod backends;
mod command;
mod value;

use std::future::Future;

use crate::{command::Command, value::Value};

/// The conglomerate error type for all [`kraglin`](crate) commands.
#[derive(Debug, Clone, thiserror::Error)]
pub enum KraglinError {
  #[error("This value is the wrong type.")]
  WrongType,
  #[error("This string type could not be parsed as an integer.")]
  CannotParseAsInteger,
  #[error("This value is out of range")]
  OutOfRange,
}

/// The generalized backend trait. All storage/execution backends implement
/// this.
pub trait Backend: Send + Sync + 'static {
  fn execute(
    &self,
    command: Command,
  ) -> impl Future<Output = Result<Value, KraglinError>> + Send;
}

#[tokio::main]
async fn main() {
  println!("Hello, world!");
}
