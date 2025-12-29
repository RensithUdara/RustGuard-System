//! Data models for RustGuard

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// User account information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct User {
    pub id: String,
    pub username: String,
    pub email: String,
    pub password_hash: String,
    pub public_key: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

/// File metadata stored on server
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileMetadata {
    pub id: String,
    pub user_id: String,
    pub path: String,
    pub name: String,
    pub size: u64,
    pub encrypted_hash: String,
    pub chunk_count: u32,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub is_deleted: bool,
}

/// File chunk for incremental sync
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileChunk {
    pub id: String,
    pub file_id: String,
    pub chunk_index: u32,
    pub encrypted_data: Vec<u8>,
    pub size: u32,
    pub hash: String,
}

/// File version for rollback capability
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileVersion {
    pub id: String,
    pub file_id: String,
    pub version_number: u32,
    pub size: u64,
    pub created_at: DateTime<Utc>,
    pub created_by: String,
}

/// Sync configuration for a directory
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SyncDirectory {
    pub id: String,
    pub user_id: String,
    pub local_path: String,
    pub remote_path: String,
    pub enabled: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

/// API Request/Response types
#[derive(Debug, Serialize, Deserialize)]
pub struct LoginRequest {
    pub username: String,
    pub password: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LoginResponse {
    pub token: String,
    pub user_id: String,
    pub username: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RegisterRequest {
    pub username: String,
    pub email: String,
    pub password: String,
    pub public_key: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UploadFileRequest {
    pub path: String,
    pub size: u64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UploadChunkRequest {
    pub file_id: String,
    pub chunk_index: u32,
    pub encrypted_data: Vec<u8>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SyncStatus {
    pub file_id: String,
    pub status: String,
    pub progress: u32,
    pub error: Option<String>,
}
