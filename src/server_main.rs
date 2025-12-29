//! Server entry point for RustGuard

use rust_guard::error::Result;
use rust_guard::server;
use rust_guard::server::db::Database;
use std::sync::Arc;
use tracing::info;

pub async fn run_server() -> Result<()> {
    info!("Starting RustGuard Server v{}", rust_guard::VERSION);

    // Initialize database
    let db = Arc::new(Database::new("sqlite:rustguard.db").await?);
    info!("Database initialized");

    // Create app
    let app = server::create_app(db).await?;

    // Run server
    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .map_err(|e| rust_guard::error::Error::Internal(e.to_string()))?;

    info!("Server listening on http://127.0.0.1:3000");

    axum::serve(listener, app)
        .await
        .map_err(|e| rust_guard::error::Error::Internal(e.to_string()))?;

    Ok(())
}
