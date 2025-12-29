# RustGuard - Secure Distributed File Backup & Sync System

A lightweight, end-to-end encrypted file backup and synchronization system written in Rust. RustGuard provides zero-knowledge cloud storage capabilities similar to Google Drive or Dropbox, but self-hosted and fully encrypted.

## 🌟 Features

### Core Features
- **End-to-End Encryption**: AES-256-GCM encryption with client-side key management
- **User Authentication**: JWT-based authentication with secure password hashing (Argon2)
- **File Management**: Upload, download, delete, and version control
- **Sync Engine**: Automatic detection of file changes with conflict resolution
- **CLI Client**: Full-featured command-line interface
- **REST API Server**: Axum-based REST API for file operations
- **Database Integration**: SQLite database for metadata storage
- **File Chunking**: Support for large files through chunk-based upload/download
- **Version Control**: Maintain file history and restore previous versions

### Advanced Features
- Local file watching for automatic sync
- Differential sync (only changed chunks)
- Multi-conflict resolution strategies
- Secure token-based API authentication
- Structured logging and error handling
- S3-compatible object storage support (stub)

## 🏗️ Project Structure

```
rust_guard/
├── src/
│   ├── main.rs                 # CLI entry point
│   ├── lib.rs                  # Library root
│   ├── server_main.rs          # Server entry point
│   ├── error.rs                # Error types
│   ├── models.rs               # Data models
│   ├── crypto.rs               # Encryption/decryption
│   ├── sync.rs                 # File synchronization
│   ├── storage.rs              # Storage abstraction (local/S3)
│   ├── client/                 # Client module
│   │   ├── mod.rs
│   │   ├── cli.rs              # CLI commands
│   │   ├── config.rs           # Configuration
│   │   └── sync_client.rs      # Client sync logic
│   └── server/                 # Server module
│       ├── mod.rs
│       ├── api.rs              # API module stub
│       ├── auth.rs             # JWT authentication
│       ├── db.rs               # Database layer
│       └── handlers.rs         # HTTP handlers
├── Cargo.toml                  # Dependencies
└── README.md                   # This file
```

## 📦 Dependencies

Key crates used:
- **tokio**: Async runtime for server and client
- **axum**: Web framework for REST API
- **sqlx**: Database access with SQLite
- **aes-gcm**: Encryption
- **argon2**: Password hashing
- **jsonwebtoken**: JWT authentication
- **clap**: CLI argument parsing
- **notify**: File watching
- **serde**: Serialization/deserialization

See `Cargo.toml` for full dependency list.

## 🚀 Quick Start

### Building the Project

```bash
cd rust_guard
cargo build --release
```

### Running the Server

```bash
cargo run -- server
```

The server will start on `http://127.0.0.1:3000`

### Using the CLI Client

```bash
# Show help
cargo run -- help

# Register a new account
cargo run -- register --username myuser --email user@example.com --password mypass

# Login
cargo run -- login --username myuser --password mypass

# Add a directory to sync
cargo run -- add-sync --path /home/user/Documents --remote /documents

# List synced directories
cargo run -- list-sync

# Start sync daemon
cargo run -- sync

# Check sync status
cargo run -- status

# Download a file
cargo run -- download --file-id abc123 --output /home/user/Downloads

# List all files
cargo run -- list

# Show version
cargo run -- version
```

## 🔐 Security Highlights

### Encryption
- **AES-256-GCM**: Military-grade symmetric encryption for file content
- **Argon2**: Memory-hard password hashing resistant to brute-force attacks
- **Client-side Keys**: Encryption keys never sent to server
- **Server-side**: Only encrypted data stored; server cannot read raw files

### Authentication
- **JWT Tokens**: 7-day expiring tokens for API access
- **Password Hashing**: Argon2 with random salts
- **Token Verification**: Automatic token validation on protected endpoints

### Privacy
- **Zero-Knowledge**: Server never has access to encryption keys
- **Private Metadata**: File paths and names encrypted
- **No Tracking**: No user activity logging

## 🛠️ Core Modules

### `crypto.rs`
Handles all cryptographic operations:
- `encrypt()`: AES-256-GCM encryption
- `decrypt()`: AES-256-GCM decryption
- `hash_password()`: Argon2 password hashing
- `verify_password()`: Password verification
- `compute_hash()`: SHA256 file hashing
- `encrypt_large_file()`: Chunk-based encryption

### `sync.rs`
File synchronization engine:
- `FileWatcher`: Detects file changes using notify crate
- `FileChangeEvent`: Enum for different change types
- `compute_file_hash()`: Hash for change detection
- `detect_changes()`: Check if file changed
- `ConflictResolution`: Strategy for handling conflicts
- `resolve_conflict()`: Resolve file conflicts

### `storage.rs`
Storage abstraction layer:
- `LocalStorage`: File storage on local filesystem
- `ObjectStorage`: S3-compatible cloud storage (stub)
- Async file operations with `tokio::fs`
- Directory traversal with `walkdir`

### `server/`
Backend server implementation:
- **api.rs**: API endpoints definition
- **auth.rs**: JWT token generation and verification
- **db.rs**: SQLite database operations
- **handlers.rs**: HTTP request handlers
- **mod.rs**: Server initialization and routing

### `client/`
CLI client implementation:
- **cli.rs**: Command-line argument parsing
- **config.rs**: Configuration file management
- **sync_client.rs**: File upload/download logic
- **mod.rs**: Client initialization

## 📊 API Endpoints

### Authentication
- `POST /api/v1/auth/register` - Register new user
- `POST /api/v1/auth/login` - Login and get JWT token
- `GET /api/v1/auth/verify` - Verify token validity

### Files
- `POST /api/v1/files/upload` - Initiate file upload
- `GET /api/v1/files/download/:file_id` - Download file
- `GET /api/v1/files/list` - List user's files
- `POST /api/v1/files/delete/:file_id` - Delete file

### Chunks
- `POST /api/v1/chunks/upload` - Upload file chunk
- `GET /api/v1/chunks/download/:chunk_id` - Download chunk

### Sync
- `GET /api/v1/sync/status` - Get sync status
- `GET /api/v1/sync/directories` - List sync directories
- `POST /api/v1/sync/directories` - Add sync directory

### Versions
- `GET /api/v1/versions/:file_id` - List file versions
- `POST /api/v1/versions/:file_id/restore/:version_id` - Restore version

### Health
- `GET /health` - Server health check

## 🧪 Testing

Run tests with:

```bash
cargo test
```

Tests included for:
- Encryption/decryption
- Password hashing/verification
- File hashing
- Local storage operations
- File watching

## 📋 Configuration

### Server Configuration
Create `rustguard.db` for SQLite (auto-created)

### Client Configuration
Configuration stored in `~/.config/rustguard/config.toml` with:
- Server URL
- User credentials
- Synced directories

### Environment Variables
- `RUST_LOG`: Set logging level (e.g., `RUST_LOG=rust_guard=debug`)

## 🔄 Sync Workflow

1. **Watch Files**: Monitor local directory for changes
2. **Detect Changes**: Compare file hashes with remote versions
3. **Encrypt**: Encrypt changed files with AES-256-GCM
4. **Upload**: Send encrypted chunks to server
5. **Store Metadata**: Save file metadata in database
6. **Verify**: Confirm successful upload
7. **Resolve Conflicts**: Handle simultaneous edits

## 🚧 Implementation Status

### Completed ✅
- [x] Project structure and modules
- [x] Crypto module with AES-256-GCM
- [x] User authentication with JWT
- [x] SQLite database schema
- [x] REST API server with Axum
- [x] CLI commands
- [x] File storage abstraction
- [x] Sync engine framework
- [x] Error handling

### In Progress 🔄
- [ ] HTTP client for API calls (upgrade to reqwest)
- [ ] Complete file upload/download implementation
- [ ] File watcher using notify crate properly
- [ ] Database operations for files/chunks
- [ ] S3 storage integration
- [ ] Web dashboard (optional)

### Future Enhancements 📈
- [ ] P2P synchronization
- [ ] Differential compression
- [ ] Multi-user collaboration
- [ ] Mobile client
- [ ] Web UI
- [ ] Rate limiting
- [ ] Detailed audit logs
- [ ] Backup scheduling

## 🤝 Contributing

To contribute to RustGuard:

1. Create a feature branch
2. Make your changes
3. Write tests
4. Ensure `cargo build` and `cargo test` pass
5. Submit a pull request

## 📝 License

MIT License - See LICENSE file for details

## 🎯 Use Cases

- **Personal Cloud Storage**: Self-hosted alternative to Google Drive
- **Secure Backups**: Encrypted backups of important files
- **Team Collaboration**: Shared encrypted workspace
- **Compliance**: HIPAA/GDPR-compliant file storage
- **Privacy-First**: Protect sensitive data from snooping

## ⚠️ Security Notes

1. **Change the JWT secret** in `src/server/auth.rs` before production
2. **Use HTTPS** for server communication in production
3. **Rotate API keys** periodically
4. **Keep Rust updated** for security patches
5. **Review crypto implementations** before production use

## 📞 Support

For issues, questions, or suggestions, please open an issue on GitHub.

---

**RustGuard**: Securing your files with the power of Rust! 🦀
