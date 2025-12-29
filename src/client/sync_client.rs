//! Sync client for uploading and downloading files

use crate::crypto;
use crate::error::Result;
use std::path::Path;
use tokio::fs;

/// Client for syncing files with server
pub struct SyncClient {
    server_url: String,
    token: String,
}

impl SyncClient {
    /// Creates a new sync client
    pub fn new(server_url: String, token: String) -> Self {
        Self { server_url, token }
    }

    /// Uploads a file to the server
    pub async fn upload_file(&self, file_path: &Path, encryption_key: &[u8; 32]) -> Result<String> {
        // Read file
        let data = fs::read(file_path).await?;

        // Encrypt file
        let (encrypted, nonce) = crypto::encrypt(&data, encryption_key)?;

        // Compute hash
        let hash = crypto::compute_hash(&encrypted);

        // TODO: Send to server via HTTP
        // For now, return the hash as file_id
        Ok(hash)
    }

    /// Downloads a file from the server
    pub async fn download_file(&self, file_id: &str, output_path: &Path, encryption_key: &[u8; 32]) -> Result<()> {
        // TODO: Download from server via HTTP

        // Decrypt and save
        let _decrypted = vec![]; // placeholder
        fs::write(output_path, _decrypted).await?;

        Ok(())
    }

    /// Lists files on server
    pub async fn list_files(&self) -> Result<Vec<String>> {
        // TODO: Call server API
        Ok(vec![])
    }
}
