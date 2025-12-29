//! CLI commands for RustGuard client

use crate::error::Result;
use clap::{Parser, Subcommand};
use dialoguer::Input;
use std::path::PathBuf;

#[derive(Parser)]
#[command(name = "RustGuard")]
#[command(about = "Secure distributed file backup and sync system", long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Register a new account
    Register {
        #[arg(short, long)]
        username: Option<String>,

        #[arg(short, long)]
        email: Option<String>,

        #[arg(short, long)]
        password: Option<String>,
    },

    /// Login to your account
    Login {
        #[arg(short, long)]
        username: Option<String>,

        #[arg(short, long)]
        password: Option<String>,
    },

    /// Add a directory to sync
    AddSync {
        #[arg(short, long)]
        path: Option<PathBuf>,

        #[arg(short, long)]
        remote: Option<String>,
    },

    /// List synced directories
    ListSync,

    /// Start sync daemon
    Sync,

    /// Show sync status
    Status,

    /// Download a file
    Download {
        #[arg(short, long)]
        file_id: String,

        #[arg(short, long)]
        output: Option<PathBuf>,
    },

    /// List files
    List,

    /// Show version
    Version,

    /// Show help
    Help,
}

impl Cli {
    /// Parses CLI arguments
    pub fn parse_args() -> Result<Self> {
        Ok(Self::parse())
    }
}

/// Interactive input helper
pub fn prompt_input(message: &str) -> Result<String> {
    Input::new()
        .with_prompt(message)
        .interact()
        .map_err(|e| crate::error::Error::Internal(e.to_string()))
}

/// Secure password input helper
pub fn prompt_password(message: &str) -> Result<String> {
    dialoguer::Password::new()
        .with_prompt(message)
        .interact()
        .map_err(|e| crate::error::Error::Internal(e.to_string()))
}
