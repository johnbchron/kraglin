#![feature(ascii_char)]

mod backends;
mod command;
mod value;

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

#[tokio::main]
async fn main() {
  println!("Hello, world!");
}
