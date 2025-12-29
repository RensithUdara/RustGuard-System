//! Client configuration management

use crate::error::Result;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserConfig {
    pub username: String,
    pub email: String,
    pub token: String,
    pub user_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SyncDirConfig {
    pub path: String,
    pub remote_path: String,
    pub enabled: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClientConfigFile {
    pub server_url: String,
    pub user: Option<UserConfig>,
    pub sync_directories: Vec<SyncDirConfig>,
}

impl ClientConfigFile {
    /// Creates a new empty config
    pub fn new(server_url: String) -> Self {
        Self {
            server_url,
            user: None,
            sync_directories: Vec::new(),
        }
    }

    /// Loads config from file
    pub fn load(path: &Path) -> Result<Self> {
        let content = fs::read_to_string(path)
            .map_err(|e| crate::error::Error::ConfigError(e.to_string()))?;

        toml::from_str(&content)
            .map_err(|e| crate::error::Error::ConfigError(e.to_string()))
    }

    /// Saves config to file
    pub fn save(&self, path: &Path) -> Result<()> {
        let content = toml::to_string_pretty(self)
            .map_err(|e| crate::error::Error::ConfigError(e.to_string()))?;

        fs::write(path, content)
            .map_err(|e| crate::error::Error::ConfigError(e.to_string()))
    }
}
