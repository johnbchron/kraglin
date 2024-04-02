use std::future::Future;

use crate::{command::Command, value::Value, KraglinError};

mod simple;

/// The generalized backend trait. All storage/execution backends implement
/// this.
pub trait Backend: Send + Sync + 'static {
  fn execute(
    &self,
    command: Command,
  ) -> impl Future<Output = Result<Value, KraglinError>> + Send;
}
