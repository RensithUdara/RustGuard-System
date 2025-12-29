//! RustGuard - Secure Distributed File Backup & Sync System
//!
//! A lightweight, end-to-end encrypted file backup and synchronization system
//! written in Rust, providing zero-knowledge cloud storage capabilities.

pub mod client;
pub mod crypto;
pub mod error;
pub mod models;
pub mod server;
pub mod storage;
pub mod sync;

pub use error::{Error, Result};

/// Version information
pub const VERSION: &str = env!("CARGO_PKG_VERSION");
