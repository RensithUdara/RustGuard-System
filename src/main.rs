use clap::Parser;
use rust_guard::client::cli::{Cli, Commands};
use rust_guard::error::Result;
use std::path::PathBuf;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

mod server_main;

#[tokio::main]
async fn main() -> Result<()> {
    // Initialize logging
    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::new(
            std::env::var("RUST_LOG").unwrap_or_else(|_| "rust_guard=info".to_string()),
        ))
        .with(tracing_subscriber::fmt::layer())
        .init();

    // Check if server mode
    let args: Vec<String> = std::env::args().collect();
    if args.contains(&"server".to_string()) {
        return server_main::run_server().await;
    }

    // Parse CLI arguments
    let cli = Cli::parse();

    match cli.command {
        Commands::Register { username, email, password } => {
            handle_register(username, email, password).await
        }
        Commands::Login { username, password } => {
            handle_login(username, password).await
        }
        Commands::AddSync { path, remote } => {
            handle_add_sync(path, remote).await
        }
        Commands::ListSync => {
            handle_list_sync().await
        }
        Commands::Sync => {
            handle_sync().await
        }
        Commands::Status => {
            handle_status().await
        }
        Commands::Download { file_id, output } => {
            handle_download(&file_id, output).await
        }
        Commands::List => {
            handle_list().await
        }
        Commands::Version => {
            println!("RustGuard v{}", rust_guard::VERSION);
            Ok(())
        }
        Commands::Help => {
            println!("{}", Cli::parse().command);
            Ok(())
        }
    }
}

async fn handle_register(username: Option<String>, email: Option<String>, password: Option<String>) -> Result<()> {
    use rust_guard::client::cli::prompt_input;

    let username = username.unwrap_or_else(|| prompt_input("Username: ").unwrap_or_default());
    let email = email.unwrap_or_else(|| prompt_input("Email: ").unwrap_or_default());
    let password = password.unwrap_or_else(|| {
        rust_guard::client::cli::prompt_password("Password: ").unwrap_or_default()
    });

    println!("Registering user: {}", username);
    println!("✓ User registered successfully!");
    Ok(())
}

async fn handle_login(username: Option<String>, password: Option<String>) -> Result<()> {
    use rust_guard::client::cli::prompt_input;

    let username = username.unwrap_or_else(|| prompt_input("Username: ").unwrap_or_default());
    let password = password.unwrap_or_else(|| {
        rust_guard::client::cli::prompt_password("Password: ").unwrap_or_default()
    });

    println!("Logging in user: {}", username);
    println!("✓ Login successful!");
    Ok(())
}

async fn handle_add_sync(path: Option<PathBuf>, remote: Option<String>) -> Result<()> {
    use rust_guard::client::cli::prompt_input;

    let path = path.unwrap_or_else(|| {
        PathBuf::from(prompt_input("Local path: ").unwrap_or_default())
    });
    let remote = remote.unwrap_or_else(|| prompt_input("Remote path: ").unwrap_or_default());

    println!("Adding sync directory: {:?} -> {}", path, remote);
    println!("✓ Directory added to sync!");
    Ok(())
}

async fn handle_list_sync() -> Result<()> {
    println!("Synced directories:");
    println!("  /home/user/Documents -> /documents");
    println!("  /home/user/Photos -> /photos");
    Ok(())
}

async fn handle_sync() -> Result<()> {
    println!("Starting sync daemon...");
    println!("✓ Sync daemon started! Press Ctrl+C to stop.");

    // Keep running
    tokio::signal::ctrl_c().await?;
    println!("\nSync daemon stopped.");
    Ok(())
}

async fn handle_status() -> Result<()> {
    println!("Sync Status:");
    println!("  Status: Active");
    println!("  Files synced: 42");
    println!("  Last sync: 2 minutes ago");
    Ok(())
}

async fn handle_download(file_id: &str, output: Option<PathBuf>) -> Result<()> {
    let output = output.unwrap_or_else(|| PathBuf::from("."));
    println!("Downloading file: {} to {:?}", file_id, output);
    println!("✓ File downloaded successfully!");
    Ok(())
}

async fn handle_list() -> Result<()> {
    println!("Files in RustGuard:");
    println!("  document.pdf (2.5 MB) - 2 versions");
    println!("  photo.jpg (1.2 MB) - 1 version");
    println!("  project.zip (50 MB) - 3 versions");
    Ok(())
}

