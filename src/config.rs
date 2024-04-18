//! Application-wide configuration.
use std::borrow::Cow;

use color_eyre::eyre::{Result, WrapErr};

/// Application-wide configuration.
///
/// # Settings
/// - `listen_port`: the port the application will listen on for TCP
///   connections. Taken from env var `LISTEN_PORT`, defaults to `6379`.
/// - `listen_host`: the host descriptor the application will listen on for TCP
///   connections. Taken from env var `LISTEN_HOST`, defaults to `0.0.0.0`.
pub struct Config {
  listen_port: usize,
  listen_host: Cow<'static, str>,
}

impl Config {
  /// Returns the port the application will listen on for TCP connections.
  pub fn listen_port(&self) -> usize { self.listen_port }
  /// Returns the host descriptor the application will listen on for TCP
  /// connections.
  pub fn listen_host(&self) -> Cow<'static, str> { self.listen_host.clone() }
}

impl Config {
  /// Builds the config from environment variables.
  ///
  /// This function will only fail if `LISTEN_PORT` cannot be parse to a
  /// `usize`.
  pub fn from_env() -> Result<Config> {
    let config = Config {
      listen_port: std::env::var("LISTEN_PORT")
        .unwrap_or("6379".to_string())
        .parse()
        .wrap_err("failed to parse `LISTEN_PORT` from env var")?,
      listen_host: std::env::var("LISTEN_HOST")
        .unwrap_or("0.0.0.0".to_string())
        .into(),
    };
    Ok(config)
  }
}
