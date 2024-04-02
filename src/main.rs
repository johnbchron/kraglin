mod command;
mod value;

use std::future::Future;

use tokio::sync::oneshot;

use crate::{command::Command, value::Value};

/// The conglomerate error type for all [`kraglin`](crate) commands.
#[derive(Debug, Clone, thiserror::Error)]
pub enum KraglinError {}

/// A convenience type alias for a oneshot receiver with a [`kraglin`](crate)
/// result.
type ResultReceiver = oneshot::Receiver<Result<Value, KraglinError>>;

/// The generalized backend trait. All storage/execution backends implement
/// this.
pub trait Backend: Send + Sync + 'static {
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
