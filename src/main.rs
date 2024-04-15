#![feature(ascii_char)]

use std::borrow::Cow;

use color_eyre::eyre::{Result, WrapErr};
use tokio::{
  io::AsyncWriteExt,
  net::{TcpListener, TcpStream},
};

pub mod backends;
pub mod command;
pub mod value;

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

/// Alias for `Result<Value, KraglinError>`
pub type KraglinResult = Result<value::Value, KraglinError>;

pub struct Settings {
  listen_port: usize,
  listen_host: Cow<'static, str>,
}

impl Settings {
  pub fn from_env() -> Result<Settings> {
    let settings = Settings {
      listen_port: std::env::var("LISTEN_PORT")
        .unwrap_or("6379".to_string())
        .parse()
        .wrap_err("failed to parse `LISTEN_PORT` from env var")?,
      listen_host: std::env::var("LISTEN_HOST")
        .unwrap_or("0.0.0.0".to_string())
        .into(),
    };
    Ok(settings)
  }
}

fn setup_tracing() {
  use tracing_error::ErrorLayer;
  use tracing_subscriber::{fmt, prelude::*, EnvFilter};

  let fmt_layer = fmt::layer().with_thread_ids(true).with_target(false);
  let filter_layer = EnvFilter::try_from_default_env()
    .or_else(|_| EnvFilter::try_new("kraglin=debug,info"))
    .unwrap();

  tracing_subscriber::registry()
    .with(filter_layer)
    .with(fmt_layer)
    .with(ErrorLayer::default())
    .init();
}

#[tokio::main]
async fn main() -> Result<()> {
  setup_tracing();

  let settings = Settings::from_env()?;

  let listen_address =
    format!("{}:{}", settings.listen_host, settings.listen_port);
  let listener = TcpListener::bind(&listen_address)
    .await
    .wrap_err("failed to create TCP listener")?;
  tracing::info!("listening on {listen_address}");

  loop {
    tracing::debug!("waiting for new connection");
    let (stream, addr) = listener
      .accept()
      .await
      .wrap_err("failed to accept TCP connection")?;
    tracing::info!("accepted connection from {addr}");
    tokio::spawn(async move { process_stream(stream).await });
  }
}

async fn process_stream(mut stream: TcpStream) -> Result<()> {
  let mut buf = vec![0; 1024];

  // In a loop, read data from the socket and write the data back.
  loop {
    let n = stream
      .try_read(&mut buf)
      .wrap_err("failed to read data from socket")?;

    if n == 0 {
      return Ok(());
    }

    stream
      .write_all(&buf[0..n])
      .await
      .wrap_err("failed to write data to socket")?;
  }
}
