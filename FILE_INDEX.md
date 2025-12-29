# RustGuard - Complete File Index

## 📁 Project File Structure

```
rust_guard/
├── Cargo.toml                          # Project manifest & dependencies
├── Cargo.lock                          # Locked dependency versions
│
├── README.md                           # Project overview & usage
├── GETTING_STARTED.md                  # Quick start guide  
├── ARCHITECTURE.md                     # System design & architecture
├── SECURITY.md                         # Security documentation
├── PROJECT_SUMMARY.md                  # Completion summary
│
└── src/
    ├── lib.rs                          # Library root & module declarations
    ├── main.rs                         # CLI entry point (9 commands)
    ├── server_main.rs                  # Server startup
    ├── error.rs                        # Custom error types (23 variants)
    ├── models.rs                       # Data structures (User, File, etc.)
    ├── crypto.rs                       # Encryption/decryption (AES-256-GCM)
    ├── sync.rs                         # File synchronization & watching
    ├── storage.rs                      # File storage abstraction
    │
    ├── client/                         # CLI Client Module (3 files)
    │   ├── mod.rs                      # Client initialization & config
    │   ├── cli.rs                      # Command definitions (clap)
    │   ├── config.rs                   # Configuration management
    │   └── sync_client.rs              # File upload/download logic
    │
    └── server/                         # Backend Server Module (5 files)
        ├── mod.rs                      # Server setup & routing (17 endpoints)
        ├── api.rs                      # API module stub
        ├── auth.rs                     # JWT authentication
        ├── db.rs                       # SQLite database operations
        └── handlers.rs                 # HTTP request handlers
```

## 📄 File Descriptions

### Configuration Files

#### `Cargo.toml` (50 lines)
Project manifest specifying:
- Package metadata
- Edition: 2021
- 50+ dependencies for:
  - Async runtime (tokio)
  - Web framework (axum)
  - Cryptography (aes-gcm, argon2, ring)
  - Database (sqlx, sqlite)
  - CLI (clap, dialoguer)
  - And more...

#### `Cargo.lock`
Auto-generated dependency lock file for reproducible builds.

### Documentation Files

#### `README.md` (500+ lines)
**Start here!** Contains:
- Feature overview
- Project structure
- Quick start commands
- API endpoint documentation
- Security highlights
- Configuration guide
- Use cases

#### `GETTING_STARTED.md` (400+ lines)
Hands-on tutorial with:
- Prerequisites and setup
- Step-by-step CLI usage
- API testing examples
- Code reading order
- Development tips
- Troubleshooting

#### `ARCHITECTURE.md` (600+ lines)
Deep dive into design:
- System architecture diagrams
- Data flow documentation
- Database schema (SQL)
- Module descriptions
- Development workflow
- Performance considerations
- Roadmap

#### `SECURITY.md` (500+ lines)
Security implementation:
- Cryptographic guarantees
- Authentication & authorization
- Data security (at rest & in transit)
- Privacy protections
- Threat model
- Deployment checklist
- Vulnerability reporting

#### `PROJECT_SUMMARY.md` (400+ lines)
Completion summary:
- What was built
- Statistics and metrics
- Key features checklist
- Code quality assessment
- Future enhancements
- Learning outcomes

### Source Code Files

#### `src/lib.rs` (30 lines)
Library root:
- Module declarations (7 public modules)
- VERSION constant
- Crate documentation

#### `src/main.rs` (150 lines)
CLI client entry point:
- Command-line argument parsing
- 10 command handlers
- User interaction prompts
- Help text

#### `src/server_main.rs` (30 lines)
Server startup:
- Database initialization
- Axum server setup
- Port configuration

#### `src/error.rs` (60 lines)
Error handling:
- 23 custom error variants
- Result<T> type alias
- Error context messages

#### `src/models.rs` (150 lines)
Data structures:
- User account info
- File metadata
- File chunks
- Versions
- Sync directories
- API request/response DTOs

#### `src/crypto.rs` (150 lines)
Cryptographic operations:
- AES-256-GCM encryption/decryption
- Argon2 password hashing
- Password verification
- SHA256 file hashing
- Nonce generation
- Includes tests

#### `src/sync.rs` (150 lines)
File synchronization:
- FileWatcher for change detection
- FileChangeEvent types
- File hash computation
- Change detection logic
- Conflict resolution strategies
- Includes tests

#### `src/storage.rs` (180 lines)
Storage abstraction:
- LocalStorage (filesystem)
- ObjectStorage (S3-compatible)
- Async file operations
- Directory management
- Includes tests

#### `src/client/mod.rs` (50 lines)
Client module root:
- Sub-module declarations
- ClientConfig struct
- Configuration defaults

#### `src/client/cli.rs` (80 lines)
CLI command definitions:
- Commands enum (9 variants)
- Clap attribute macros
- Input prompts
- Password input handling

#### `src/client/config.rs` (80 lines)
Configuration management:
- UserConfig struct
- SyncDirConfig struct
- TOML serialization
- File load/save operations

#### `src/client/sync_client.rs` (60 lines)
File synchronization client:
- SyncClient struct
- File upload logic
- File download logic
- File listing

#### `src/server/mod.rs` (60 lines)
Server module root:
- ServerState struct
- Router creation
- 17 API endpoints
- Middleware setup

#### `src/server/api.rs` (2 lines)
API placeholder:
- Reserved for future API organization

#### `src/server/auth.rs` (60 lines)
JWT authentication:
- Claims struct
- Token generation
- Token verification
- Middleware extraction

#### `src/server/db.rs` (280 lines)
SQLite database:
- Database struct
- Connection pooling
- Schema initialization (5 tables)
- CRUD operations:
  - create_user()
  - get_user_by_username()
  - create_file_metadata()
  - list_user_files()

#### `src/server/handlers.rs` (250 lines)
HTTP request handlers:
- Authentication (register, login, verify)
- File operations (upload, download, list, delete)
- Chunk operations (upload, download)
- Sync operations (status, directories)
- Version operations (list, restore)
- Health check

## 📊 Statistics

### Code Files (18 total)
| Category | Count | Lines |
|----------|-------|-------|
| Server modules | 5 | 650 |
| Client modules | 4 | 270 |
| Core modules | 8 | 920 |
| **Total** | **17** | **~1,840** |

### Documentation Files (5 total)
| File | Lines | Purpose |
|------|-------|---------|
| README.md | 500+ | Overview & quick start |
| GETTING_STARTED.md | 400+ | Tutorial & setup |
| ARCHITECTURE.md | 600+ | System design |
| SECURITY.md | 500+ | Security details |
| PROJECT_SUMMARY.md | 400+ | Status & metrics |

### Total Project
- **Source files**: 18
- **Documentation**: 5
- **Configuration**: 2
- **Total lines of code**: ~1,840
- **Total documentation**: ~2,400 lines
- **Total project lines**: ~4,240 lines

## 🎯 Where to Start

### For Users
1. Read `README.md` - understand what RustGuard is
2. Follow `GETTING_STARTED.md` - hands-on tutorial
3. Try CLI commands - build muscle memory
4. Review `SECURITY.md` - understand security model

### For Developers
1. Read `README.md` - feature overview
2. Read `ARCHITECTURE.md` - system design
3. Start with `src/lib.rs` - module organization
4. Review `src/models.rs` - data structures
5. Study `src/crypto.rs` - core logic
6. Explore `src/server/` - API implementation
7. Check `src/client/` - CLI implementation

### For Auditors/Reviewers
1. `SECURITY.md` - security model & threat analysis
2. `src/crypto.rs` - cryptographic implementation
3. `src/server/auth.rs` - authentication
4. `src/server/db.rs` - database security
5. `src/server/handlers.rs` - API security
6. `src/error.rs` - error handling

### For Learners
1. `GETTING_STARTED.md` - setup & examples
2. `src/main.rs` - CLI entry point (simple)
3. `src/models.rs` - data structures
4. `src/crypto.rs` - cryptography (with tests)
5. `src/sync.rs` - synchronization
6. `src/server/handlers.rs` - web API

## 🔗 File Dependencies

```
main.rs
├── client/cli.rs
├── client/mod.rs
├── server_main.rs
└── (all modules)

server_main.rs
├── server/mod.rs
├── server/db.rs
└── error.rs

server/mod.rs
├── server/handlers.rs
├── server/auth.rs
└── server/db.rs

server/handlers.rs
├── crypto.rs
├── models.rs
├── server/auth.rs
└── error.rs

crypto.rs (no internal deps)

sync.rs (no internal deps)

storage.rs (no internal deps)

client/sync_client.rs
├── crypto.rs
└── error.rs
```

## 📦 External Dependencies (50+)

Key dependencies by file:

### `crypto.rs`
- aes-gcm: AES encryption
- argon2: Password hashing
- password-hash: Hashing utilities
- sha2: SHA256
- rand: Random generation

### `server/mod.rs`
- axum: Web framework
- tower-http: HTTP middleware
- tokio: Async runtime

### `server/db.rs`
- sqlx: Database access
- chrono: Date/time
- uuid: Unique IDs

### `server/auth.rs`
- jsonwebtoken: JWT handling
- serde: Serialization

### `client/cli.rs`
- clap: Argument parsing
- dialoguer: User prompts

### `client/config.rs`
- toml: TOML files
- serde: Serialization

### `sync.rs`
- notify: File watching

### `storage.rs`
- tokio::fs: Async file I/O
- walkdir: Directory traversal

## 🧪 Testing Structure

Test modules included in:
- `src/crypto.rs`: 3 tests
  - `test_encrypt_decrypt`
  - `test_hash_password`
  - `test_compute_hash`

- `src/sync.rs`: 3 tests
  - `test_file_watcher_creation`
  - `test_add_path`
  - `test_compute_file_hash`

- `src/storage.rs`: 2 tests
  - `test_local_storage_creation`
  - `test_store_and_retrieve`

Run tests with: `cargo test`

## 📈 Project Size Growth

| Phase | Files | LOC | Modules |
|-------|-------|-----|---------|
| Initial | 1 | 2 | - |
| After setup | 2 | 50 | - |
| Core modules | 8 | 920 | 7 |
| Client | 12 | 1,190 | 9 |
| Server | 17 | 1,840 | 12 |
| **Final** | **18** | **1,840** | **16** |

## 🎓 Learning Path

Reading sequence for understanding RustGuard:

1. **Conceptual** (30 min)
   - README.md: What & why

2. **Design** (30 min)
   - ARCHITECTURE.md: How

3. **Setup** (15 min)
   - GETTING_STARTED.md: Installation

4. **Practical** (30 min)
   - Try CLI commands
   - Test API endpoints

5. **Code Structure** (30 min)
   - lib.rs: Module organization
   - models.rs: Data structures
   - main.rs: CLI entry

6. **Core Logic** (60 min)
   - crypto.rs: Encryption
   - sync.rs: Synchronization
   - storage.rs: File handling

7. **Server** (60 min)
   - server/mod.rs: Setup
   - server/auth.rs: JWT
   - server/db.rs: Database
   - server/handlers.rs: API

8. **Advanced** (Variable)
   - SECURITY.md: Detailed security
   - Extend with your features

---

**Happy exploring!** 🚀

All files are in `e:\RustGuard\rust_guard\`
