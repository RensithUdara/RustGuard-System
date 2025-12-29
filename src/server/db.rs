//! Database module for RustGuard

use crate::error::{Error, Result};
use crate::models::*;
use chrono::Utc;
use sqlx::sqlite::{SqliteConnectOptions, SqlitePool, SqlitePoolOptions};
use std::str::FromStr;
use uuid::Uuid;

/// Database connection pool
pub struct Database {
    pool: SqlitePool,
}

impl Database {
    /// Creates a new database instance
    pub async fn new(database_url: &str) -> Result<Self> {
        let options = SqliteConnectOptions::from_str(database_url)
            .map_err(|e| Error::DatabaseError(e.to_string()))?
            .create_if_missing(true);

        let pool = SqlitePoolOptions::new()
            .max_connections(5)
            .connect_with(options)
            .await
            .map_err(|e| Error::DatabaseError(e.to_string()))?;

        let db = Self { pool };
        db.init_schema().await?;
        Ok(db)
    }

    /// Initializes database schema
    async fn init_schema(&self) -> Result<()> {
        sqlx::query(
            r#"
            CREATE TABLE IF NOT EXISTS users (
                id TEXT PRIMARY KEY,
                username TEXT UNIQUE NOT NULL,
                email TEXT UNIQUE NOT NULL,
                password_hash TEXT NOT NULL,
                public_key TEXT NOT NULL,
                created_at TEXT NOT NULL,
                updated_at TEXT NOT NULL
            )
            "#,
        )
        .execute(&self.pool)
        .await
        .map_err(|e| Error::DatabaseError(e.to_string()))?;

        sqlx::query(
            r#"
            CREATE TABLE IF NOT EXISTS file_metadata (
                id TEXT PRIMARY KEY,
                user_id TEXT NOT NULL,
                path TEXT NOT NULL,
                name TEXT NOT NULL,
                size INTEGER NOT NULL,
                encrypted_hash TEXT NOT NULL,
                chunk_count INTEGER NOT NULL,
                created_at TEXT NOT NULL,
                updated_at TEXT NOT NULL,
                is_deleted BOOLEAN NOT NULL DEFAULT 0,
                FOREIGN KEY (user_id) REFERENCES users(id)
            )
            "#,
        )
        .execute(&self.pool)
        .await
        .map_err(|e| Error::DatabaseError(e.to_string()))?;

        sqlx::query(
            r#"
            CREATE TABLE IF NOT EXISTS file_chunks (
                id TEXT PRIMARY KEY,
                file_id TEXT NOT NULL,
                chunk_index INTEGER NOT NULL,
                encrypted_data BLOB NOT NULL,
                size INTEGER NOT NULL,
                hash TEXT NOT NULL,
                FOREIGN KEY (file_id) REFERENCES file_metadata(id)
            )
            "#,
        )
        .execute(&self.pool)
        .await
        .map_err(|e| Error::DatabaseError(e.to_string()))?;

        sqlx::query(
            r#"
            CREATE TABLE IF NOT EXISTS file_versions (
                id TEXT PRIMARY KEY,
                file_id TEXT NOT NULL,
                version_number INTEGER NOT NULL,
                size INTEGER NOT NULL,
                created_at TEXT NOT NULL,
                created_by TEXT NOT NULL,
                FOREIGN KEY (file_id) REFERENCES file_metadata(id)
            )
            "#,
        )
        .execute(&self.pool)
        .await
        .map_err(|e| Error::DatabaseError(e.to_string()))?;

        sqlx::query(
            r#"
            CREATE TABLE IF NOT EXISTS sync_directories (
                id TEXT PRIMARY KEY,
                user_id TEXT NOT NULL,
                local_path TEXT NOT NULL,
                remote_path TEXT NOT NULL,
                enabled BOOLEAN NOT NULL DEFAULT 1,
                created_at TEXT NOT NULL,
                updated_at TEXT NOT NULL,
                FOREIGN KEY (user_id) REFERENCES users(id)
            )
            "#,
        )
        .execute(&self.pool)
        .await
        .map_err(|e| Error::DatabaseError(e.to_string()))?;

        Ok(())
    }

    /// Creates a new user
    pub async fn create_user(&self, username: &str, email: &str, password_hash: &str, public_key: &str) -> Result<User> {
        let id = Uuid::new_v4().to_string();
        let now = Utc::now();

        sqlx::query(
            "INSERT INTO users (id, username, email, password_hash, public_key, created_at, updated_at) VALUES (?, ?, ?, ?, ?, ?, ?)"
        )
        .bind(&id)
        .bind(username)
        .bind(email)
        .bind(password_hash)
        .bind(public_key)
        .bind(now.to_rfc3339())
        .bind(now.to_rfc3339())
        .execute(&self.pool)
        .await
        .map_err(|e| Error::DatabaseError(e.to_string()))?;

        Ok(User {
            id,
            username: username.to_string(),
            email: email.to_string(),
            password_hash: password_hash.to_string(),
            public_key: public_key.to_string(),
            created_at: now,
            updated_at: now,
        })
    }

    /// Retrieves a user by username
    pub async fn get_user_by_username(&self, username: &str) -> Result<Option<User>> {
        let user = sqlx::query_as::<_, (String, String, String, String, String, String, String)>(
            "SELECT id, username, email, password_hash, public_key, created_at, updated_at FROM users WHERE username = ?"
        )
        .bind(username)
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| Error::DatabaseError(e.to_string()))?;

        Ok(user.map(|(id, username, email, password_hash, public_key, created_at, updated_at)| {
            User {
                id,
                username,
                email,
                password_hash,
                public_key,
                created_at: created_at.parse().unwrap_or_else(|_| Utc::now()),
                updated_at: updated_at.parse().unwrap_or_else(|_| Utc::now()),
            }
        }))
    }

    /// Creates file metadata
    pub async fn create_file_metadata(&self, user_id: &str, path: &str, name: &str, size: u64, encrypted_hash: &str, chunk_count: u32) -> Result<FileMetadata> {
        let id = Uuid::new_v4().to_string();
        let now = Utc::now();

        sqlx::query(
            "INSERT INTO file_metadata (id, user_id, path, name, size, encrypted_hash, chunk_count, created_at, updated_at) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?)"
        )
        .bind(&id)
        .bind(user_id)
        .bind(path)
        .bind(name)
        .bind(size as i64)
        .bind(encrypted_hash)
        .bind(chunk_count as i32)
        .bind(now.to_rfc3339())
        .bind(now.to_rfc3339())
        .execute(&self.pool)
        .await
        .map_err(|e| Error::DatabaseError(e.to_string()))?;

        Ok(FileMetadata {
            id,
            user_id: user_id.to_string(),
            path: path.to_string(),
            name: name.to_string(),
            size,
            encrypted_hash: encrypted_hash.to_string(),
            chunk_count,
            created_at: now,
            updated_at: now,
            is_deleted: false,
        })
    }

    /// Lists all files for a user
    pub async fn list_user_files(&self, user_id: &str) -> Result<Vec<FileMetadata>> {
        let files = sqlx::query_as::<_, (String, String, String, String, i64, String, i32, String, String, bool)>(
            "SELECT id, user_id, path, name, size, encrypted_hash, chunk_count, created_at, updated_at, is_deleted FROM file_metadata WHERE user_id = ? AND is_deleted = 0"
        )
        .bind(user_id)
        .fetch_all(&self.pool)
        .await
        .map_err(|e| Error::DatabaseError(e.to_string()))?;

        Ok(files.into_iter().map(|(id, user_id, path, name, size, encrypted_hash, chunk_count, created_at, updated_at, is_deleted)| {
            FileMetadata {
                id,
                user_id,
                path,
                name,
                size: size as u64,
                encrypted_hash,
                chunk_count: chunk_count as u32,
                created_at: created_at.parse().unwrap_or_else(|_| Utc::now()),
                updated_at: updated_at.parse().unwrap_or_else(|_| Utc::now()),
                is_deleted,
            }
        }).collect())
    }
}
