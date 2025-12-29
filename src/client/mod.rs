//! Client module for RustGuard CLI

pub mod cli;
pub mod config;
pub mod sync_client;

use crate::error::Result;
use std::path::PathBuf;

/// Client configuration
pub struct ClientConfig {
    pub server_url: String,
    pub data_dir: PathBuf,
    pub config_file: PathBuf,
}

impl ClientConfig {
    /// Creates a new client config with defaults
    pub fn new() -> Result<Self> {
        let data_dir = dirs::data_dir()
            .ok_or_else(|| crate::error::Error::ConfigError("Cannot find data directory".to_string()))?
            .join("rustguard");

        Ok(Self {
            server_url: "http://localhost:3000".to_string(),
            data_dir: data_dir.clone(),
            config_file: data_dir.join("config.toml"),
        })
    }
}

impl Default for ClientConfig {
    fn default() -> Self {
        Self::new().unwrap_or_else(|_| Self {
            server_url: "http://localhost:3000".to_string(),
            data_dir: PathBuf::from("/tmp/rustguard"),
            config_file: PathBuf::from("/tmp/rustguard/config.toml"),
        })
    }
}
