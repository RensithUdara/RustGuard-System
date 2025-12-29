//! API handlers for RustGuard server

use crate::crypto;
use crate::error::{Error, Result};
use crate::models::*;
use crate::server::{auth, db::Database};
use axum::{
    extract::{Path, State},
    http::{HeaderMap, StatusCode},
    response::IntoResponse,
    Json,
};
use serde_json::json;
use std::sync::Arc;
use tracing::info;

pub type ServerState = Arc<crate::server::ServerState>;

/// Health check endpoint
pub async fn health() -> impl IntoResponse {
    (StatusCode::OK, "RustGuard Server v0.1.0")
}

/// User registration endpoint
pub async fn register(
    State(state): State<ServerState>,
    Json(req): Json<RegisterRequest>,
) -> impl IntoResponse {
    match _register(&state, &req).await {
        Ok(response) => (StatusCode::CREATED, Json(response)).into_response(),
        Err(e) => {
            let msg = json!({"error": e.to_string()});
            (StatusCode::BAD_REQUEST, Json(msg)).into_response()
        }
    }
}

async fn _register(state: &ServerState, req: &RegisterRequest) -> Result<serde_json::Value> {
    // Hash password
    let password_hash = crypto::hash_password(&req.password)?;

    // Create user in database
    let user = state
        .db
        .create_user(&req.username, &req.email, &password_hash, &req.public_key)
        .await?;

    // Generate token
    let token = auth::generate_token(&user.id)?;

    info!("User registered: {}", req.username);

    Ok(json!({
        "token": token,
        "user_id": user.id,
        "username": user.username,
        "email": user.email
    }))
}

/// User login endpoint
pub async fn login(
    State(state): State<ServerState>,
    Json(req): Json<LoginRequest>,
) -> impl IntoResponse {
    match _login(&state, &req).await {
        Ok(response) => (StatusCode::OK, Json(response)).into_response(),
        Err(e) => {
            let msg = json!({"error": e.to_string()});
            (StatusCode::UNAUTHORIZED, Json(msg)).into_response()
        }
    }
}

async fn _login(state: &ServerState, req: &LoginRequest) -> Result<serde_json::Value> {
    // Get user
    let user = state
        .db
        .get_user_by_username(&req.username)
        .await?
        .ok_or(Error::UserNotFound)?;

    // Verify password
    if !crypto::verify_password(&req.password, &user.password_hash)? {
        return Err(Error::InvalidCredentials);
    }

    // Generate token
    let token = auth::generate_token(&user.id)?;

    info!("User logged in: {}", req.username);

    Ok(json!({
        "token": token,
        "user_id": user.id,
        "username": user.username
    }))
}

/// Token verification endpoint
pub async fn verify_token(headers: HeaderMap) -> impl IntoResponse {
    match headers.get("authorization") {
        Some(header_value) => {
            let token = header_value
                .to_str()
                .unwrap_or("")
                .strip_prefix("Bearer ")
                .unwrap_or("");

            match auth::verify_token(token) {
                Ok(claims) => (
                    StatusCode::OK,
                    Json(json!({"valid": true, "user_id": claims.sub})),
                )
                    .into_response(),
                Err(_) => (
                    StatusCode::UNAUTHORIZED,
                    Json(json!({"valid": false})),
                )
                    .into_response(),
            }
        }
        None => (
            StatusCode::UNAUTHORIZED,
            Json(json!({"valid": false})),
        )
            .into_response(),
    }
}

/// File upload endpoint
pub async fn upload_file(
    State(state): State<ServerState>,
    headers: HeaderMap,
    Json(req): Json<UploadFileRequest>,
) -> impl IntoResponse {
    match _upload_file(&state, &headers, &req).await {
        Ok(response) => (StatusCode::CREATED, Json(response)).into_response(),
        Err(e) => {
            let msg = json!({"error": e.to_string()});
            (StatusCode::BAD_REQUEST, Json(msg)).into_response()
        }
    }
}

async fn _upload_file(
    state: &ServerState,
    headers: &HeaderMap,
    req: &UploadFileRequest,
) -> Result<serde_json::Value> {
    let token = headers
        .get("authorization")
        .and_then(|h| h.to_str().ok())
        .and_then(|h| h.strip_prefix("Bearer "))
        .ok_or(Error::AuthenticationFailed("Missing token".to_string()))?;

    let claims = crate::server::auth::verify_token(token)?;
    let user_id = claims.sub;

    // Compute hash for file
    let hash = crypto::compute_hash(req.path.as_bytes());

    // Create file metadata
    let file = state
        .db
        .create_file_metadata(&user_id, &req.path, "", req.size, &hash, 0)
        .await?;

    info!("File uploaded: {} by user {}", req.path, user_id);

    Ok(json!({
        "file_id": file.id,
        "path": file.path,
        "size": file.size
    }))
}

/// File download endpoint
pub async fn download_file(
    State(_state): State<ServerState>,
    Path(_file_id): Path<String>,
) -> impl IntoResponse {
    (StatusCode::OK, "File download not yet implemented").into_response()
}

/// List files endpoint
pub async fn list_files(
    State(state): State<ServerState>,
    headers: HeaderMap,
) -> impl IntoResponse {
    match _list_files(&state, &headers).await {
        Ok(files) => (StatusCode::OK, Json(json!({"files": files}))).into_response(),
        Err(e) => {
            let msg = json!({"error": e.to_string()});
            (StatusCode::UNAUTHORIZED, Json(msg)).into_response()
        }
    }
}

async fn _list_files(state: &ServerState, headers: &HeaderMap) -> Result<Vec<FileMetadata>> {
    let token = headers
        .get("authorization")
        .and_then(|h| h.to_str().ok())
        .and_then(|h| h.strip_prefix("Bearer "))
        .ok_or(Error::AuthenticationFailed("Missing token".to_string()))?;

    let claims = crate::server::auth::verify_token(token)?;
    state.db.list_user_files(&claims.sub).await
}

/// Delete file endpoint
pub async fn delete_file(
    State(_state): State<ServerState>,
    Path(_file_id): Path<String>,
) -> impl IntoResponse {
    (StatusCode::OK, Json(json!({"deleted": true}))).into_response()
}

/// Upload chunk endpoint
pub async fn upload_chunk(
    State(_state): State<ServerState>,
    Json(_req): Json<UploadChunkRequest>,
) -> impl IntoResponse {
    (StatusCode::CREATED, Json(json!({"chunk_id": "chunk_123"}))).into_response()
}

/// Download chunk endpoint
pub async fn download_chunk(
    State(_state): State<ServerState>,
    Path(_chunk_id): Path<String>,
) -> impl IntoResponse {
    (StatusCode::OK, "Chunk download not yet implemented").into_response()
}

/// Sync status endpoint
pub async fn sync_status() -> impl IntoResponse {
    (StatusCode::OK, Json(json!({"status": "synced"}))).into_response()
}

/// List sync directories endpoint
pub async fn list_sync_dirs(
    State(_state): State<ServerState>,
    headers: HeaderMap,
) -> impl IntoResponse {
    match headers.get("authorization") {
        Some(_) => (StatusCode::OK, Json(json!({"directories": []})))
            .into_response(),
        None => (StatusCode::UNAUTHORIZED, Json(json!({"error": "Unauthorized"})))
            .into_response(),
    }
}

/// Add sync directory endpoint
pub async fn add_sync_dir(
    State(_state): State<ServerState>,
    Json(_req): Json<SyncDirectory>,
) -> impl IntoResponse {
    (StatusCode::CREATED, Json(json!({"directory_id": "dir_123"})))
        .into_response()
}

/// List versions endpoint
pub async fn list_versions(
    State(_state): State<ServerState>,
    Path(_file_id): Path<String>,
) -> impl IntoResponse {
    (StatusCode::OK, Json(json!({"versions": []})))
        .into_response()
}

/// Restore version endpoint
pub async fn restore_version(
    State(_state): State<ServerState>,
    Path((_file_id, _version_id)): Path<(String, String)>,
) -> impl IntoResponse {
    (StatusCode::OK, Json(json!({"restored": true})))
        .into_response()
}
