//! Server module for RustGuard

pub mod api;
pub mod auth;
pub mod db;
pub mod handlers;

use crate::error::Result;
use axum::{
    middleware,
    routing::{get, post},
    Router,
};
use std::sync::Arc;
use tower_http::cors::CorsLayer;
use tracing::info;

/// Server state containing shared resources
pub struct ServerState {
    pub db: Arc<db::Database>,
}

/// Initializes and returns the application router
pub async fn create_app(db: Arc<db::Database>) -> Result<Router> {
    let state = Arc::new(ServerState { db });

    let app = Router::new()
        // Health check
        .route("/health", get(handlers::health))
        // Auth endpoints
        .route("/api/v1/auth/register", post(handlers::register))
        .route("/api/v1/auth/login", post(handlers::login))
        .route("/api/v1/auth/verify", get(handlers::verify_token))
        // File endpoints
        .route("/api/v1/files/upload", post(handlers::upload_file))
        .route("/api/v1/files/download/:file_id", get(handlers::download_file))
        .route("/api/v1/files/list", get(handlers::list_files))
        .route("/api/v1/files/delete/:file_id", post(handlers::delete_file))
        // Chunk endpoints
        .route("/api/v1/chunks/upload", post(handlers::upload_chunk))
        .route("/api/v1/chunks/download/:chunk_id", get(handlers::download_chunk))
        // Sync endpoints
        .route("/api/v1/sync/status", get(handlers::sync_status))
        .route("/api/v1/sync/directories", get(handlers::list_sync_dirs))
        .route("/api/v1/sync/directories", post(handlers::add_sync_dir))
        // Versioning endpoints
        .route("/api/v1/versions/:file_id", get(handlers::list_versions))
        .route("/api/v1/versions/:file_id/restore/:version_id", post(handlers::restore_version))
        .layer(middleware::from_fn(auth::extract_token))
        .layer(CorsLayer::permissive())
        .with_state(state);

    info!("RustGuard server initialized");
    Ok(app)
}
