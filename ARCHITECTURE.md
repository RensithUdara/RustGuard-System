# RustGuard Architecture & Development Guide

## System Architecture

```
┌─────────────────────────────────────────────────────────────┐
│                       CLI Client (Rust)                      │
│  ┌──────────────────────────────────────────────────────┐   │
│  │  User Commands → CLI Parser → Command Handlers       │   │
│  │  Config Management | Authentication | File Sync      │   │
│  └──────────────────────────────────────────────────────┘   │
└──────────────────┬──────────────────────────────────────────┘
                   │ HTTP/REST
                   ▼
┌─────────────────────────────────────────────────────────────┐
│                    Server (Rust - Axum)                      │
│  ┌──────────────────────────────────────────────────────┐   │
│  │  HTTP Handlers                                        │   │
│  │  ├─ Auth: Register, Login, Token Verify             │   │
│  │  ├─ Files: Upload, Download, List, Delete           │   │
│  │  ├─ Chunks: Upload/Download chunks                  │   │
│  │  ├─ Sync: Status, Directories                       │   │
│  │  └─ Versions: List, Restore                         │   │
│  └──────────────────────────────────────────────────────┘   │
│  ┌──────────────────────────────────────────────────────┐   │
│  │  Authentication Layer (JWT)                          │   │
│  │  ├─ Token Generation                                │   │
│  │  ├─ Token Validation                                │   │
│  │  └─ Middleware Integration                          │   │
│  └──────────────────────────────────────────────────────┘   │
└──────────────────┬──────────────────────────────────────────┘
                   │
          ┌────────┴────────┐
          ▼                 ▼
    ┌─────────────┐  ┌──────────────────┐
    │   Storage   │  │    Encryption    │
    │   (Local)   │  │   (AES-256-GCM)  │
    └─────────────┘  └──────────────────┘
                   │
                   ▼
    ┌──────────────────────────────┐
    │    Database (SQLite)         │
    │  ├─ Users Table              │
    │  ├─ File Metadata Table       │
    │  ├─ Chunks Table             │
    │  ├─ Versions Table           │
    │  └─ Sync Directories Table   │
    └──────────────────────────────┘
```

## Data Flow

### File Upload Flow

```
User                Client                 Server
  │                   │                      │
  │─ Upload File ────>│                      │
  │                   │─ Generate Key ───┐  │
  │                   │<─────────────────┘  │
  │                   │─ Encrypt File ───┐  │
  │                   │<─────────────────┘  │
  │                   │─ Split into Chunks  │
  │                   │─ Hash Each Chunk    │
  │                   │                     │
  │                   │─ POST /upload ─────>│
  │                   │                     ├─ Validate Auth
  │                   │                     ├─ Store Encrypted
  │                   │                     ├─ Save Metadata
  │                   │<─ File ID ──────────│
  │<─ Success ────────│                     │
```

### File Download Flow

```
User                Client                 Server
  │                   │                      │
  │─ Download ──────>│                      │
  │                   │─ GET /download ───>│
  │                   │                     ├─ Validate Auth
  │                   │                     ├─ Retrieve Encrypted
  │                   │<─ Encrypted Data ──│
  │                   │─ Decrypt ─────┐    │
  │                   │<──────────────┘    │
  │                   │─ Verify Hash       │
  │<─ File ───────────│                    │
```

## Database Schema

### Users Table
```sql
CREATE TABLE users (
    id TEXT PRIMARY KEY,
    username TEXT UNIQUE NOT NULL,
    email TEXT UNIQUE NOT NULL,
    password_hash TEXT NOT NULL,
    public_key TEXT NOT NULL,
    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL
);
```

### File Metadata Table
```sql
CREATE TABLE file_metadata (
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
);
```

### File Chunks Table
```sql
CREATE TABLE file_chunks (
    id TEXT PRIMARY KEY,
    file_id TEXT NOT NULL,
    chunk_index INTEGER NOT NULL,
    encrypted_data BLOB NOT NULL,
    size INTEGER NOT NULL,
    hash TEXT NOT NULL,
    FOREIGN KEY (file_id) REFERENCES file_metadata(id)
);
```

### File Versions Table
```sql
CREATE TABLE file_versions (
    id TEXT PRIMARY KEY,
    file_id TEXT NOT NULL,
    version_number INTEGER NOT NULL,
    size INTEGER NOT NULL,
    created_at TEXT NOT NULL,
    created_by TEXT NOT NULL,
    FOREIGN KEY (file_id) REFERENCES file_metadata(id)
);
```

### Sync Directories Table
```sql
CREATE TABLE sync_directories (
    id TEXT PRIMARY KEY,
    user_id TEXT NOT NULL,
    local_path TEXT NOT NULL,
    remote_path TEXT NOT NULL,
    enabled BOOLEAN NOT NULL DEFAULT 1,
    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL,
    FOREIGN KEY (user_id) REFERENCES users(id)
);
```

## Encryption & Security

### AES-256-GCM Implementation
```rust
// Key generation
key: [u8; 32] = derive_key(password, salt)

// Encryption
(ciphertext, nonce) = encrypt(plaintext, key)

// Decryption
plaintext = decrypt(ciphertext, key, nonce)

// File verification
hash = compute_hash(data)
```

### Password Security
- Algorithm: Argon2id
- Time cost: 2
- Memory cost: 19456 KB (~19 MB)
- Parallelism: 1
- Output length: 32 bytes

### JWT Tokens
- Algorithm: HS256
- Expiration: 7 days
- Contains: user_id, expiration time
- Validated on all protected endpoints

## Module Structure

### `error.rs`
Custom error types for the application:
- `AuthenticationFailed`: Auth errors
- `EncryptionError`: Crypto failures
- `DatabaseError`: DB operations
- `StorageError`: File operations
- Result<T> type alias

### `models.rs`
Data structures:
- `User`: User account info
- `FileMetadata`: File information
- `FileChunk`: Chunk data
- `FileVersion`: Version history
- `SyncDirectory`: Sync configuration
- Request/Response DTOs

### `crypto.rs`
Cryptographic operations:
- Key derivation (Argon2)
- Password hashing & verification
- AES-256-GCM encryption/decryption
- SHA256 hashing
- Nonce generation

### `sync.rs`
Synchronization engine:
- File watching
- Change detection
- Conflict resolution
- Hash computation
- Event handling

### `storage.rs`
Storage abstraction:
- Local filesystem storage
- S3-compatible storage (stub)
- Async file operations
- File listing & management

### `server/`
Backend server components:
- `auth.rs`: JWT handling
- `db.rs`: Database operations
- `handlers.rs`: HTTP request processing
- `mod.rs`: Server setup & routing

### `client/`
CLI client components:
- `cli.rs`: Command definitions
- `config.rs`: Configuration management
- `sync_client.rs`: Sync operations
- `mod.rs`: Client initialization

## Running & Development

### Development Build
```bash
cargo build
cargo run -- help
```

### Production Build
```bash
cargo build --release
./target/release/rust_guard server
```

### Testing
```bash
# Run all tests
cargo test

# Run specific test
cargo test test_encrypt_decrypt

# Run with output
cargo test -- --nocapture

# Run tests with logging
RUST_LOG=debug cargo test -- --nocapture
```

### Debugging
```bash
# Enable debug logging
RUST_LOG=rust_guard=debug cargo run -- help

# With backtrace
RUST_BACKTRACE=1 cargo run -- login
```

## Development Workflow

### Adding a New CLI Command

1. Add variant to `Commands` enum in `client/cli.rs`
2. Parse arguments with clap attributes
3. Create handler function in `main.rs`
4. Implement command logic

Example:
```rust
#[derive(Subcommand)]
pub enum Commands {
    // ... existing commands
    NewCommand {
        #[arg(short, long)]
        option: Option<String>,
    },
}

// In main.rs
Commands::NewCommand { option } => {
    handle_new_command(option).await
}
```

### Adding a New API Endpoint

1. Add route in `server/mod.rs`
2. Create handler in `server/handlers.rs`
3. Add database operation in `server/db.rs`
4. Update models if needed
5. Add tests

Example:
```rust
.route("/api/v1/new/endpoint", post(handlers::new_endpoint))
```

### Adding a New Database Table

1. Add CREATE TABLE in `server/db.rs::init_schema()`
2. Create model struct in `models.rs`
3. Add CRUD operations in `server/db.rs`
4. Create handler functions

## Performance Considerations

### Optimization Strategies
1. **Chunking**: Split large files into 1MB chunks
2. **Lazy Loading**: Load file metadata on demand
3. **Connection Pooling**: SQLx connection pool (5 connections)
4. **Async I/O**: All I/O operations are async
5. **Caching**: Token validation caching (future)

### Benchmarking
```bash
# Time test execution
time cargo test

# Profile with release build
cargo build --release
time ./target/release/rust_guard --help
```

## Deployment

### Docker (Future)
```dockerfile
FROM rust:latest
WORKDIR /app
COPY . .
RUN cargo build --release
EXPOSE 3000
CMD ["./target/release/rust_guard", "server"]
```

### System Requirements
- CPU: 2+ cores recommended
- RAM: 512 MB minimum
- Disk: Depends on usage
- OS: Linux, macOS, Windows

### Environment Setup
1. Install Rust: `curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh`
2. Clone repository
3. Run `cargo build --release`
4. Configure database path
5. Set environment variables

## Contributing Guidelines

### Code Style
- Follow Rust conventions
- Use meaningful variable names
- Add doc comments to public functions
- Keep functions focused and small

### Testing Requirements
- Add tests for new functionality
- Ensure all tests pass: `cargo test`
- Maintain >80% code coverage

### Documentation
- Update README for user-facing changes
- Add inline comments for complex logic
- Document public APIs

## Roadmap

### v0.2.0 (Next)
- [ ] Implement reqwest for HTTP client
- [ ] Complete file upload/download
- [ ] Proper file watching
- [ ] Database full CRUD

### v0.3.0
- [ ] S3 integration
- [ ] Web dashboard
- [ ] Batch operations
- [ ] Differential sync

### v1.0.0 (Release)
- [ ] Full feature parity
- [ ] Performance optimization
- [ ] Security audit
- [ ] Production deployment

---

**Happy coding!** 🦀
