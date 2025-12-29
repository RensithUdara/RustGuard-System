//! Error types for RustGuard

use thiserror::Error;

/// Result type for RustGuard operations
pub type Result<T> = std::result::Result<T, Error>;

/// Error types for RustGuard
#[derive(Error, Debug)]
pub enum Error {
    #[error("Authentication failed: {0}")]
    AuthenticationFailed(String),

    #[error("Invalid credentials")]
    InvalidCredentials,

    #[error("User not found")]
    UserNotFound,

    #[error("File not found: {0}")]
    FileNotFound(String),

    #[error("Encryption error: {0}")]
    EncryptionError(String),

    #[error("Decryption error: {0}")]
    DecryptionError(String),

    #[error("Database error: {0}")]
    DatabaseError(String),

    #[error("I/O error: {0}")]
    IoError(#[from] std::io::Error),

    #[error("Serialization error: {0}")]
    SerializationError(String),

    #[error("Invalid input: {0}")]
    InvalidInput(String),

    #[error("Sync error: {0}")]
    SyncError(String),

    #[error("Storage error: {0}")]
    StorageError(String),

    #[error("Configuration error: {0}")]
    ConfigError(String),

    #[error("Network error: {0}")]
    NetworkError(String),

    #[error("Conflict resolution failed: {0}")]
    ConflictError(String),

    #[error("Internal error: {0}")]
    Internal(String),
}
