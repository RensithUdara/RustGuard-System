//! Storage layer for file management

use crate::error::{Error, Result};
use std::fs;
use std::path::{Path, PathBuf};
use tokio::fs as async_fs;

/// Local file storage handler
pub struct LocalStorage {
    base_path: PathBuf,
}

impl LocalStorage {
    /// Creates a new local storage instance
    pub fn new(base_path: PathBuf) -> Result<Self> {
        // Create directory if it doesn't exist
        fs::create_dir_all(&base_path)
            .map_err(|e| Error::StorageError(e.to_string()))?;

        Ok(Self { base_path })
    }

    /// Stores a file locally
    pub async fn store_file(&self, relative_path: &str, data: &[u8]) -> Result<PathBuf> {
        let file_path = self.base_path.join(relative_path);

        // Create parent directories
        if let Some(parent) = file_path.parent() {
            async_fs::create_dir_all(parent)
                .await
                .map_err(|e| Error::StorageError(e.to_string()))?;
        }

        async_fs::write(&file_path, data)
            .await
            .map_err(|e| Error::StorageError(e.to_string()))?;

        Ok(file_path)
    }

    /// Retrieves a file from storage
    pub async fn retrieve_file(&self, relative_path: &str) -> Result<Vec<u8>> {
        let file_path = self.base_path.join(relative_path);

        async_fs::read(&file_path)
            .await
            .map_err(|e| Error::StorageError(e.to_string()))
    }

    /// Deletes a file from storage
    pub async fn delete_file(&self, relative_path: &str) -> Result<()> {
        let file_path = self.base_path.join(relative_path);

        async_fs::remove_file(&file_path)
            .await
            .map_err(|e| Error::StorageError(e.to_string()))
    }

    /// Lists all files in storage
    pub fn list_files(&self) -> Result<Vec<PathBuf>> {
        let mut files = Vec::new();

        for entry in walkdir::WalkDir::new(&self.base_path)
            .into_iter()
            .filter_map(|e| e.ok())
        {
            if entry.path().is_file() {
                files.push(entry.path().to_path_buf());
            }
        }

        Ok(files)
    }

    /// Gets file size
    pub fn get_file_size(&self, relative_path: &str) -> Result<u64> {
        let file_path = self.base_path.join(relative_path);

        fs::metadata(&file_path)
            .map(|m| m.len())
            .map_err(|e| Error::StorageError(e.to_string()))
    }

    /// Checks if file exists
    pub fn file_exists(&self, relative_path: &str) -> bool {
        self.base_path.join(relative_path).exists()
    }
}

/// S3-compatible object storage handler (stub)
pub struct ObjectStorage {
    bucket: String,
    // In a real implementation, this would contain S3 client
}

impl ObjectStorage {
    /// Creates a new object storage instance
    pub fn new(bucket: String) -> Self {
        Self { bucket }
    }

    /// Uploads file to object storage
    pub async fn upload_file(&self, key: &str, _data: &[u8]) -> Result<()> {
        // TODO: Implement actual S3 upload
        tracing::info!("Would upload {} to S3 bucket {}", key, self.bucket);
        Ok(())
    }

    /// Downloads file from object storage
    pub async fn download_file(&self, key: &str) -> Result<Vec<u8>> {
        // TODO: Implement actual S3 download
        tracing::info!("Would download {} from S3 bucket {}", key, self.bucket);
        Ok(vec![])
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_local_storage_creation() {
        let temp_dir = tempfile::tempdir().unwrap();
        let storage = LocalStorage::new(temp_dir.path().to_path_buf()).unwrap();
        assert!(storage.base_path.exists());
    }

    #[tokio::test]
    async fn test_store_and_retrieve() {
        let temp_dir = tempfile::tempdir().unwrap();
        let storage = LocalStorage::new(temp_dir.path().to_path_buf()).unwrap();

        let data = b"test file content";
        let path = storage.store_file("test.txt", data).await.unwrap();
        assert!(path.exists());

        let retrieved = storage.retrieve_file("test.txt").await.unwrap();
        assert_eq!(retrieved, data);
    }
}
