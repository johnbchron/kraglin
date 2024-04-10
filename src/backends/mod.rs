use std::future::Future;

use smol_str::SmolStr;

use crate::{command::Command, value::Value, KraglinError};

mod simple;

type KraglinResult = Result<Value, KraglinError>;

/// The generalized backend trait. All storage/execution backends implement
/// this.
pub trait Backend: Send + Sync + 'static {
  fn new() -> Self;

  fn execute(
    &self,
    command: Command,
  ) -> impl Future<Output = KraglinResult> + Send;
}

#[allow(non_snake_case)]
pub trait BackendExt: Backend {
  async fn SET(&self, key: impl Into<SmolStr>, value: Value) -> KraglinResult;
  async fn GET(&self, key: impl Into<SmolStr>) -> KraglinResult;
  async fn MGET(&self, keys: Vec<SmolStr>) -> KraglinResult;
  async fn INCR(&self, key: impl Into<SmolStr>) -> KraglinResult;
  async fn KEYS(&self) -> KraglinResult;
  async fn EXISTS(&self, key: impl Into<SmolStr>) -> KraglinResult;
  async fn DEL(&self, key: impl Into<SmolStr>) -> KraglinResult;
  async fn INFO(&self) -> KraglinResult;
  async fn HSET(
    &self,
    key: impl Into<SmolStr>,
    field: impl Into<SmolStr>,
    value: Value,
  ) -> KraglinResult;
  async fn HGET(
    &self,
    key: impl Into<SmolStr>,
    field: impl Into<SmolStr>,
  ) -> KraglinResult;
  async fn HGETALL(&self, key: impl Into<SmolStr>) -> KraglinResult;
  async fn HMGET(
    &self,
    key: impl Into<SmolStr>,
    fields: Vec<SmolStr>,
  ) -> KraglinResult;
  async fn SADD(&self, key: impl Into<SmolStr>, value: Value) -> KraglinResult;
  async fn SMEMBERS(&self, key: impl Into<SmolStr>) -> KraglinResult;
  async fn SCARD(&self, key: impl Into<SmolStr>) -> KraglinResult;
  async fn SDIFF(
    &self,
    set_a: impl Into<SmolStr>,
    set_b: impl Into<SmolStr>,
  ) -> KraglinResult;
  async fn SDIFFSTORE(
    self,
    set_a: impl Into<SmolStr>,
    set_b: impl Into<SmolStr>,
    new_set: impl Into<SmolStr>,
  ) -> KraglinResult;
  async fn SREM(&self, key: impl Into<SmolStr>, value: Value) -> KraglinResult;
  async fn LPUSH(&self, key: impl Into<SmolStr>, value: Value)
    -> KraglinResult;
  async fn RPUSH(&self, key: impl Into<SmolStr>, value: Value)
    -> KraglinResult;
  async fn LRANGE(
    &self,
    key: impl Into<SmolStr>,
    start: i64,
    end: i64,
  ) -> KraglinResult;
  async fn LLEN(&self, key: impl Into<SmolStr>) -> KraglinResult;
  async fn LPOP(&self, key: impl Into<SmolStr>) -> KraglinResult;
  async fn RPOP(&self, key: impl Into<SmolStr>) -> KraglinResult;
}

impl<B: Backend> BackendExt for B {
  async fn SET(&self, key: impl Into<SmolStr>, value: Value) -> KraglinResult {
    self
      .execute(Command::Set {
        key: key.into(),
        value,
      })
      .await
  }
  async fn GET(&self, key: impl Into<SmolStr>) -> KraglinResult {
    self.execute(Command::Get { key: key.into() }).await
  }
  async fn MGET(&self, keys: Vec<SmolStr>) -> KraglinResult {
    self.execute(Command::MultipleGet { keys }).await
  }
  async fn INCR(&self, key: impl Into<SmolStr>) -> KraglinResult {
    self.execute(Command::Increment { key: key.into() }).await
  }
  async fn KEYS(&self) -> KraglinResult { self.execute(Command::Keys).await }
  async fn EXISTS(&self, key: impl Into<SmolStr>) -> KraglinResult {
    self.execute(Command::Exists { key: key.into() }).await
  }
  async fn DEL(&self, key: impl Into<SmolStr>) -> KraglinResult {
    self.execute(Command::Delete { key: key.into() }).await
  }
  async fn INFO(&self) -> KraglinResult { self.execute(Command::Info).await }
  async fn HSET(
    &self,
    key: impl Into<SmolStr>,
    field: impl Into<SmolStr>,
    value: Value,
  ) -> KraglinResult {
    self
      .execute(Command::HashSet {
        key: key.into(),
        field: field.into(),
        value,
      })
      .await
  }
  async fn HGET(
    &self,
    key: impl Into<SmolStr>,
    field: impl Into<SmolStr>,
  ) -> KraglinResult {
    self
      .execute(Command::HashGet {
        key:   key.into(),
        field: field.into(),
      })
      .await
  }
  async fn HGETALL(&self, key: impl Into<SmolStr>) -> KraglinResult {
    self.execute(Command::HashGetAll { key: key.into() }).await
  }
  async fn HMGET(
    &self,
    key: impl Into<SmolStr>,
    fields: Vec<SmolStr>,
  ) -> KraglinResult {
    self
      .execute(Command::HashMultipleGet {
        key: key.into(),
        fields,
      })
      .await
  }
  async fn SADD(&self, key: impl Into<SmolStr>, value: Value) -> KraglinResult {
    self
      .execute(Command::SetAdd {
        key: key.into(),
        value,
      })
      .await
  }
  async fn SMEMBERS(&self, key: impl Into<SmolStr>) -> KraglinResult {
    self.execute(Command::SetMembers { key: key.into() }).await
  }
  async fn SCARD(&self, key: impl Into<SmolStr>) -> KraglinResult {
    self
      .execute(Command::SetCardinality { key: key.into() })
      .await
  }
  async fn SDIFF(
    &self,
    set_a: impl Into<SmolStr>,
    set_b: impl Into<SmolStr>,
  ) -> KraglinResult {
    self
      .execute(Command::SetDifference {
        set_a: set_a.into(),
        set_b: set_b.into(),
      })
      .await
  }
  async fn SDIFFSTORE(
    self,
    set_a: impl Into<SmolStr>,
    set_b: impl Into<SmolStr>,
    new_set: impl Into<SmolStr>,
  ) -> KraglinResult {
    self
      .execute(Command::SetDifferenceStore {
        set_a:   set_a.into(),
        set_b:   set_b.into(),
        new_set: new_set.into(),
      })
      .await
  }
  async fn SREM(&self, key: impl Into<SmolStr>, value: Value) -> KraglinResult {
    self
      .execute(Command::SetRemove {
        key: key.into(),
        value,
      })
      .await
  }
  async fn LPUSH(
    &self,
    key: impl Into<SmolStr>,
    value: Value,
  ) -> KraglinResult {
    self
      .execute(Command::LeftPush {
        key: key.into(),
        value,
      })
      .await
  }
  async fn RPUSH(
    &self,
    key: impl Into<SmolStr>,
    value: Value,
  ) -> KraglinResult {
    self
      .execute(Command::RightPush {
        key: key.into(),
        value,
      })
      .await
  }
  async fn LRANGE(
    &self,
    key: impl Into<SmolStr>,
    start: i64,
    end: i64,
  ) -> KraglinResult {
    self
      .execute(Command::ListRange {
        key: key.into(),
        start,
        end,
      })
      .await
  }
  async fn LLEN(&self, key: impl Into<SmolStr>) -> KraglinResult {
    self.execute(Command::ListLength { key: key.into() }).await
  }
  async fn LPOP(&self, key: impl Into<SmolStr>) -> KraglinResult {
    self.execute(Command::LeftPop { key: key.into() }).await
  }
  async fn RPOP(&self, key: impl Into<SmolStr>) -> KraglinResult {
    self.execute(Command::RightPop { key: key.into() }).await
  }
}

#[cfg(test)]
#[generic_tests::define(attrs(tokio::test))]
#[allow(non_snake_case)]
mod tests {
  use std::collections::BTreeMap;

  use super::{simple::SimpleBackend, Backend, BackendExt};
  use crate::{value::Value, KraglinError};

  #[tokio::test]
  async fn SET_sets_and_GET_gets<B: Backend>() -> Result<(), KraglinError> {
    let backend = B::new();

    backend
      .SET("key_a", Value::SimpleString("a".into()))
      .await?;
    assert_eq!(backend.GET("key_a").await?, Value::SimpleString("a".into()));

    Ok(())
  }

  #[tokio::test]
  async fn MGET_gets_multiple_keys<B: Backend>() -> Result<(), KraglinError> {
    let backend = B::new();

    backend.SET("key_a", Value::Integer(2)).await?;
    backend.SET("key_b", Value::Integer(4)).await?;
    assert_eq!(
      backend.MGET(vec!["key_a".into(), "key_b".into()]).await?,
      Value::Array(vec![Value::Integer(2), Value::Integer(4)])
    );

    Ok(())
  }

  #[tokio::test]
  async fn INCR_works<B: Backend>() -> Result<(), KraglinError> {
    let backend = B::new();

    backend.SET("int", Value::Integer(2)).await?;
    backend.SET("big_num", Value::BigNumber(4.into())).await?;
    backend
      .SET("string", Value::SimpleString("24".into()))
      .await?;
    backend
      .SET("bulk_string", Value::BulkString("24".into()))
      .await?;

    backend.INCR("int").await?;
    backend.INCR("big_num").await?;
    backend.INCR("string").await?;
    backend.INCR("bulk_string").await?;

    assert_eq!(backend.GET("int").await?, Value::Integer(3));
    assert_eq!(backend.GET("big_num").await?, Value::BigNumber(5.into()));
    assert_eq!(
      backend.GET("string").await?,
      Value::SimpleString("25".into())
    );
    assert_eq!(
      backend.GET("bulk_string").await?,
      Value::BulkString("25".into())
    );

    Ok(())
  }

  #[tokio::test]
  async fn KEYS_works<B: Backend>() -> Result<(), KraglinError> {
    let backend = B::new();

    backend.SET("a", Value::Integer(1)).await?;
    backend.SET("b", Value::Integer(2)).await?;

    assert_eq!(
      backend.KEYS().await?,
      Value::Array(vec![
        Value::SimpleString("a".into()),
        Value::SimpleString("b".into())
      ])
    );

    Ok(())
  }

  #[tokio::test]
  async fn EXISTS_works<B: Backend>() -> Result<(), KraglinError> {
    let backend = B::new();

    backend.SET("a", Value::Integer(1)).await?;
    assert_eq!(backend.EXISTS("a").await?, Value::Integer(1));

    assert_eq!(backend.EXISTS("b").await?, Value::Integer(0));

    Ok(())
  }

  #[tokio::test]
  async fn DELETE_works<B: Backend>() -> Result<(), KraglinError> {
    let backend = B::new();

    backend.SET("a", Value::Integer(1)).await?;
    assert_eq!(backend.EXISTS("a").await?, Value::Integer(1));
    assert_eq!(backend.DEL("a").await?, Value::Integer(1));
    assert_eq!(backend.EXISTS("a").await?, Value::Integer(0));
    assert_eq!(backend.DEL("a").await?, Value::Integer(0));

    Ok(())
  }

  #[tokio::test]
  async fn INFO_works<B: Backend>() -> Result<(), KraglinError> {
    let backend = B::new();

    backend.SET("a", Value::Integer(1)).await?;
    assert_eq!(
      backend.INFO().await?,
      Value::SimpleString(
        "We've got 1 key right now, thanks for asking :)".into()
      )
    );

    Ok(())
  }

  #[tokio::test]
  async fn HSET_sets_and_HGET_gets<B: Backend>() -> Result<(), KraglinError> {
    let backend = B::new();

    backend.HSET("a", "b", Value::Integer(1)).await?;
    assert_eq!(backend.HGET("a", "b").await?, Value::Integer(1));

    Ok(())
  }

  #[tokio::test]
  async fn HGETALL_works<B: Backend>() -> Result<(), KraglinError> {
    let backend = B::new();

    backend.HSET("a", "b", Value::Integer(1)).await?;
    backend.HSET("a", "c", Value::Integer(2)).await?;
    assert_eq!(
      backend.HGETALL("a").await?,
      Value::Map(
        [
          ("b".into(), Value::Integer(1)),
          ("c".into(), Value::Integer(2))
        ]
        .into_iter()
        .collect::<BTreeMap<smol_str::SmolStr, Value>>()
      )
    );

    Ok(())
  }

  #[tokio::test]
  async fn HMGET_works<B: Backend>() -> Result<(), KraglinError> {
    let backend = B::new();

    backend.HSET("a", "b", Value::Integer(1)).await?;
    backend.HSET("a", "c", Value::Integer(2)).await?;
    backend.HSET("a", "d", Value::Integer(3)).await?;

    assert_eq!(
      backend.HMGET("a", vec!["b".into(), "c".into()]).await?,
      Value::Array(vec![Value::Integer(1), Value::Integer(2)])
    );

    Ok(())
  }

  #[instantiate_tests(<SimpleBackend>)]
  mod simple_backend {}
}
