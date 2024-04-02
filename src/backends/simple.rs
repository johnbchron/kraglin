use std::{collections::HashMap, sync::Arc};

use smol_str::SmolStr;

use crate::{command::Command, value::Value, Backend};

pub struct SimpleBackend(Arc<HashMap<SmolStr, Value>>);

impl Backend for SimpleBackend {
  async fn execute(
    &self,
    command: Command,
    result_channel: crate::ResultSender,
  ) {
    todo!()
  }
}
