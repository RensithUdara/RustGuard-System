# Getting Started with RustGuard

Welcome to RustGuard! This guide will help you get up and running quickly.

## Prerequisites

- **Rust 1.70+**: Install from [rustup.rs](https://rustup.rs/)
- **Git**: For version control
- **3+ GB disk space**: For dependencies and build

### Verify Installation

```bash
rustc --version
cargo --version
```

Should output versions similar to:
```
rustc 1.75.0 (1d8b05fc5 2023-12-21)
cargo 1.75.0 (ecb9851af 2023-10-18)
```

## Project Setup

### 1. Navigate to Project

```bash
cd e:\RustGuard\rust_guard
```

### 2. Build the Project

```bash
# Development build (faster compilation)
cargo build

# Release build (optimized for performance)
cargo build --release
```

First build takes 1-2 minutes as Rust downloads and compiles dependencies.

### 3. Verify Build Success

```bash
cargo test
```

All tests should pass.

## Running the Application

### Option A: Development (Recommended for Learning)

Run server:
```bash
cargo run -- server
```

In another terminal, run CLI:
```bash
cargo run -- help
cargo run -- version
cargo run -- register --username demo --email demo@example.com --password demo123
```

### Option B: Release (Optimized)

```bash
# Build optimized binary
cargo build --release

# Run server
./target/release/rust_guard server

# In another terminal
./target/release/rust_guard list
```

## CLI Commands Tutorial

### 1. Register a New Account

```bash
cargo run -- register
# Follow prompts or use:
cargo run -- register --username alice --email alice@example.com --password securepass123
```

**Output:**
```
Registering user: alice
✓ User registered successfully!
```

### 2. Login

```bash
cargo run -- login --username alice --password securepass123
# Output:
# Logging in user: alice
# ✓ Login successful!
```

### 3. Add a Directory to Sync

```bash
cargo run -- add-sync --path /home/user/Documents --remote /documents
# Output:
# Adding sync directory: "/home/user/Documents" -> "/documents"
# ✓ Directory added to sync!
```

### 4. List Synced Directories

```bash
cargo run -- list-sync
# Output:
# Synced directories:
#   /home/user/Documents -> /documents
#   /home/user/Photos -> /photos
```

### 5. Start Sync Daemon

```bash
cargo run -- sync
# Output:
# Starting sync daemon...
# ✓ Sync daemon started! Press Ctrl+C to stop.
# (runs until you press Ctrl+C)
```

### 6. Check Sync Status

```bash
cargo run -- status
# Output:
# Sync Status:
#   Status: Active
#   Files synced: 42
#   Last sync: 2 minutes ago
```

### 7. List Files

```bash
cargo run -- list
# Output:
# Files in RustGuard:
#   document.pdf (2.5 MB) - 2 versions
#   photo.jpg (1.2 MB) - 1 version
#   project.zip (50 MB) - 3 versions
```

### 8. Download a File

```bash
cargo run -- download --file-id abc123def456 --output /home/user/Downloads
# Output:
# Downloading file: abc123def456 to "/home/user/Downloads"
# ✓ File downloaded successfully!
```

### 9. Show Version

```bash
cargo run -- version
# Output:
# RustGuard v0.1.0
```

### 10. Get Help

```bash
cargo run -- help
# Shows all available commands and options
```

## Server API Testing

### Start the Server

```bash
cargo run -- server
# Output:
# [INFO] Starting RustGuard Server v0.1.0
# [INFO] Database initialized
# [INFO] Server listening on http://127.0.0.1:3000
```

### Test Health Endpoint

```bash
curl http://127.0.0.1:3000/health
# Response: RustGuard Server v0.1.0
```

### Test Registration API

```bash
curl -X POST http://127.0.0.1:3000/api/v1/auth/register \
  -H "Content-Type: application/json" \
  -d '{
    "username": "testuser",
    "email": "test@example.com",
    "password": "testpass123",
    "public_key": "-----BEGIN PUBLIC KEY-----\n...\n-----END PUBLIC KEY-----"
  }'
```

### Test Login API

```bash
curl -X POST http://127.0.0.1:3000/api/v1/auth/login \
  -H "Content-Type: application/json" \
  -d '{
    "username": "testuser",
    "password": "testpass123"
  }'
```

Response:
```json
{
  "token": "eyJ0eXAiOiJKV1QiLCJhbGc...",
  "user_id": "550e8400-e29b-41d4-a716-446655440000",
  "username": "testuser"
}
```

## Understanding the Code

### Start Here: Reading Order

1. **README.md** - What is RustGuard?
2. **ARCHITECTURE.md** - How does it work?
3. **src/lib.rs** - Module organization
4. **src/main.rs** - CLI entry point
5. **src/server_main.rs** - Server entry point
6. **src/models.rs** - Data structures
7. **src/crypto.rs** - Encryption logic
8. **src/server/handlers.rs** - HTTP handlers

### Key Files Explained

| File | Purpose |
|------|---------|
| `src/lib.rs` | Module declarations and library root |
| `src/main.rs` | CLI client entry point |
| `src/error.rs` | Custom error types |
| `src/models.rs` | Data structures (User, File, etc.) |
| `src/crypto.rs` | Encryption operations |
| `src/sync.rs` | File synchronization |
| `src/storage.rs` | File storage abstraction |
| `src/client/` | CLI client code |
| `src/server/` | Server and API code |

## Development Tips

### Enable Debug Logging

```bash
RUST_LOG=debug cargo run -- help
# or for server:
RUST_LOG=rust_guard=debug cargo run -- server
```

### Run Tests with Output

```bash
cargo test -- --nocapture
# or specific test:
cargo test test_encrypt_decrypt -- --nocapture
```

### Check Code Without Compiling

```bash
cargo check
# Much faster than cargo build for catching errors
```

### Format Code

```bash
cargo fmt
# Auto-formats Rust code
```

### Lint Code

```bash
cargo clippy
# Shows potential improvements
```

### Generate Documentation

```bash
cargo doc --open
# Generates and opens local documentation
```

## Common Tasks

### Add a New CLI Command

1. Add to `enum Commands` in `src/client/cli.rs`
2. Create handler in `src/main.rs`
3. Implement logic
4. Test with `cargo run`

### Add a New API Endpoint

1. Create handler in `src/server/handlers.rs`
2. Add route in `src/server/mod.rs`
3. Database operation in `src/server/db.rs` (if needed)
4. Test with curl

### Run Specific Test

```bash
cargo test test_hash_password -- --nocapture
```

## Troubleshooting

### Build Fails

```bash
# Clean build
cargo clean
cargo build

# Check for latest Rust
rustup update
```

### Tests Fail

```bash
# Run with more details
RUST_BACKTRACE=1 cargo test -- --nocapture

# Run one test
cargo test test_name
```

### Server Won't Start

```bash
# Check if port 3000 is available
# Linux/Mac: lsof -i :3000
# Windows: netstat -ano | findstr :3000

# Use different port by modifying src/server_main.rs
```

### Performance Issues

```bash
# Use release build
cargo build --release
./target/release/rust_guard ...

# Profile with perf (Linux)
perf record cargo run --release
perf report
```

## Next Steps

### Learn Rust
- [Rust Book](https://doc.rust-lang.org/book/)
- [Rustlings Exercises](https://github.com/rust-lang/rustlings)
- [Rust by Example](https://doc.rust-lang.org/rust-by-example/)

### Understand Cryptography
- [Cryptographic Concepts](https://en.wikipedia.org/wiki/Cryptography)
- [AES-GCM Explained](https://en.wikipedia.org/wiki/Galois/Counter_Mode)
- [Password Hashing](https://cheatsheetseries.owasp.org/cheatsheets/Password_Storage_Cheat_Sheet.html)

### Expand RustGuard
- Implement HTTP file uploads with streaming
- Add S3 storage support
- Create web dashboard
- Build mobile client
- Add collaborative features

### Explore Related Projects
- [Tokio](https://tokio.rs/) - Async runtime
- [Axum](https://github.com/tokio-rs/axum) - Web framework
- [ring](https://github.com/briansmith/ring) - Cryptography
- [sqlx](https://github.com/launchbadge/sqlx) - Database

## Community & Support

### Get Help
- Check README.md for overview
- Review ARCHITECTURE.md for design
- Look at SECURITY.md for security info
- Search code with `grep` or `cargo search`

### Contribute
1. Fork the project
2. Create feature branch
3. Make changes
4. Write tests
5. Submit pull request

### Report Issues
- Describe the problem
- Include reproduction steps
- Show error messages
- Mention your environment

## Summary

You now have a working RustGuard installation with:
✅ Complete source code
✅ Executable CLI client
✅ Working REST API server
✅ SQLite database
✅ Comprehensive documentation

**Start small** - try the simple commands first, then explore the codebase.

**Have fun learning Rust!** 🦀

---

**Need help?** Check the documentation files:
- README.md - Feature overview
- ARCHITECTURE.md - System design
- SECURITY.md - Security details
- PROJECT_SUMMARY.md - What was built
