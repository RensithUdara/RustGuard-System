# RustGuard Security Guide

## Security Overview

RustGuard is designed with security as a first-class concern, implementing multiple layers of protection to ensure data confidentiality, integrity, and availability.

## Cryptographic Guarantees

### End-to-End Encryption (E2EE)

Every file uploaded to RustGuard is encrypted on the client before transmission, ensuring the server never has access to unencrypted data.

**Algorithm**: AES-256-GCM
- **Mode**: Galois/Counter Mode (GCM) - provides authenticated encryption
- **Key Size**: 256 bits (32 bytes)
- **Nonce Size**: 96 bits (12 bytes) - randomly generated per encryption
- **Tag Size**: 128 bits (16 bytes) - authentication tag

**Security Properties**:
- Provides both confidentiality and authenticity
- Resistant to timing attacks
- Hardware-accelerated on modern CPUs (AES-NI)

### Key Derivation

Password-based key derivation uses SHA256 with salt:
```
key = SHA256(password + salt)
```

For production, this should be upgraded to PBKDF2 or Argon2 for key derivation.

### Password Security

Passwords are hashed using **Argon2id**:
- **Algorithm**: Argon2id (combines Argon2i and Argon2d)
- **Memory**: 19,456 KB (~19 MB)
- **Time Cost**: 2 iterations
- **Parallelism**: 1 thread
- **Output**: 32 bytes

**Features**:
- Memory-hard: Resistant to GPU/ASIC attacks
- Adaptive: Can increase cost factors as hardware improves
- Salt**: Cryptographically random salt per password

## Authentication & Authorization

### JWT (JSON Web Tokens)

Stateless authentication using JWT:
- **Algorithm**: HS256 (HMAC-SHA256)
- **Expiration**: 7 days
- **Payload**: `{ sub: user_id, exp: timestamp }`
- **Secret**: Stored securely on server

**Best Practices**:
1. Use HTTPS for all token transmission
2. Store tokens securely (secure httponly cookies)
3. Validate token signature on every request
4. Refresh tokens regularly

### API Key Management

Protected endpoints verify authentication via:
1. Extract token from `Authorization: Bearer <token>` header
2. Validate signature using server secret
3. Check token expiration
4. Verify user existence
5. Grant or deny access

## Data Security

### At Rest

**File Storage**:
- Encrypted with AES-256-GCM
- Key stored client-side only
- Server stores only encrypted ciphertext
- File metadata encrypted with per-user key

**Database Security**:
- SQLite with encrypted database (optional: SQLCipher)
- Password hashes using Argon2id
- API tokens stored as hashes
- No sensitive data in plaintext

### In Transit

**HTTPS/TLS**:
- Required for production deployment
- Use TLS 1.2 or higher
- Strong cipher suites only
- Certificate pinning (optional)

**Data Integrity**:
- HMAC-SHA256 in GCM mode
- Chunk hashing (SHA256)
- File hash verification on download

## Privacy Protections

### Server Knows Nothing

The server cannot:
- Read file contents (encrypted)
- Determine file types (from content)
- Track access patterns (no logging of content)
- Monitor user activity in detail
- Access user passwords

### User Privacy

- No persistent session tokens
- No user activity logging
- No analytics or tracking
- No third-party cookies
- User data can be deleted on demand

## Threat Model

### Threats We Protect Against

| Threat | Solution |
|--------|----------|
| File Interception | TLS/HTTPS + E2EE |
| Server Breach | Encrypted data only |
| Password Attacks | Argon2 hashing |
| Token Forgery | HMAC signature verification |
| Man-in-the-Middle | HTTPS certificate validation |
| Timing Attacks | Constant-time comparison (GCM) |
| Replay Attacks | Token expiration (7 days) |
| File Corruption | HMAC authentication tags |

### Threats We Don't Address

| Threat | Notes |
|--------|-------|
| Quantum Computing | No post-quantum crypto yet |
| Hardware Compromise | If device is compromised, all bets are off |
| Social Engineering | User education required |
| Zero-Day Exploits | Keep Rust and dependencies updated |
| Physical Access | Full-disk encryption recommended |

## Secure Coding Practices

### Memory Safety

Rust provides memory safety guarantees:
- No buffer overflows
- No use-after-free
- No data races (without unsafe)
- No null pointer dereferences

**Crypto Implementation**:
- Uses `aes-gcm` crate (battle-tested)
- Uses `argon2` crate (well-audited)
- Minimal unsafe code
- No hardcoded secrets in code

### Input Validation

All user inputs are validated:
- File paths checked for traversal attacks
- Usernames limited to alphanumeric + underscore
- Email addresses validated
- API payloads size-limited
- Parameter types enforced

### Error Handling

Security-conscious error messages:
- Don't reveal internal details
- No SQL injection vectors
- Fail securely (deny by default)
- Log security events

## Deployment Security

### Production Checklist

- [ ] Change JWT secret in `src/server/auth.rs`
- [ ] Enable HTTPS with valid certificate
- [ ] Set `RUST_LOG` to warn/error (not debug)
- [ ] Use environment variables for secrets
- [ ] Enable database encryption (SQLCipher)
- [ ] Set up firewall rules
- [ ] Regular backups with encryption
- [ ] Monitor for suspicious activity
- [ ] Keep dependencies updated: `cargo update`

### Environment Variables

```bash
# .env file (never commit!)
JWT_SECRET=your-super-secret-key-here
DATABASE_URL=sqlite:rustguard.db
RUST_LOG=warn
```

### Docker Security

```dockerfile
FROM rust:latest as builder
# Build only

FROM debian:bullseye-slim
# Minimal runtime image
RUN useradd -m rustguard
USER rustguard
```

## Dependency Security

### Dependency Scanning

```bash
# Check for known vulnerabilities
cargo audit

# Update dependencies safely
cargo update --aggressive
cargo audit --fix
```

### Trusted Dependencies

Key dependencies and their security properties:

| Crate | Purpose | Security |
|-------|---------|----------|
| aes-gcm | AES-256 encryption | ✅ NIST standard |
| argon2 | Password hashing | ✅ OWASP recommended |
| jsonwebtoken | JWT handling | ✅ Well-audited |
| ring | Cryptography library | ✅ Audit reports available |
| tokio | Async runtime | ✅ Widely used, vetted |
| sqlx | Database access | ✅ Type-safe SQL |

## Vulnerability Reporting

### Report Security Issues

If you discover a security vulnerability:

1. **DO NOT** open a public GitHub issue
2. Email security details to: security@rustguard.dev (if available)
3. Include:
   - Description of vulnerability
   - Affected versions
   - Steps to reproduce
   - Suggested fix (optional)
4. Allow 90 days for patch before disclosure

## Security Audit

### Areas Needing Review

1. **Cryptography**: Professional audit of AES-256-GCM implementation
2. **Authentication**: JWT token lifecycle and validation
3. **Database**: SQL injection prevention and access control
4. **API**: Rate limiting, input validation, error handling
5. **Infrastructure**: Deployment security, secrets management

### Recommended Auditors

- Trail of Bits
- Cure53
- NCC Group
- Checkpoint Research

## Compliance

### Standards Alignment

- **OWASP Top 10**: Addresses known vulnerabilities
- **NIST Cybersecurity Framework**: Identifies & protects
- **ISO 27001**: Information security management
- **GDPR**: Right to be forgotten, data protection

### Data Protection

- User data encrypted at rest
- Passwords salted and hashed
- No unnecessary data retention
- User can request data deletion

## Incident Response

### If Compromised

1. **Change your password** immediately
2. **Notify server administrator**
3. **Monitor for unauthorized access**
4. **Review file access logs** (if available)
5. **Update encryption keys** for sensitive data

## Security Best Practices for Users

### Client-Side Security

1. **Use strong passwords**: 16+ characters, mixed case, numbers, symbols
2. **Keep Rust updated**: `rustup update`
3. **Secure your device**: Full-disk encryption, antivirus
4. **Use HTTPS only**: Never connect over HTTP
5. **Backup encryption keys**: Securely store your password
6. **Logout on shared devices**: Explicit token cleanup
7. **Monitor accounts**: Check for suspicious activity

### Server Administration

1. **Network Security**: Firewall, VPN, IP whitelisting
2. **Regular Updates**: Keep Rust, dependencies, OS patched
3. **Monitoring**: Log suspicious activity, set up alerts
4. **Backups**: Encrypted offsite backups
5. **Access Control**: Principle of least privilege
6. **Secrets Management**: Never hardcode secrets
7. **Rate Limiting**: Prevent brute-force attacks

## Future Security Enhancements

### Planned Improvements

- [ ] Implement PBKDF2 for key derivation
- [ ] Add TLS certificate pinning
- [ ] Rate limiting on endpoints
- [ ] Anomaly detection for unusual access
- [ ] Two-factor authentication (2FA)
- [ ] Hardware security key support
- [ ] Post-quantum cryptography
- [ ] Formal security audit

### Security Roadmap

**v0.2**:
- Upgrade key derivation
- Add comprehensive logging
- Security testing framework

**v0.3**:
- TLS certificate pinning
- Rate limiting system
- Intrusion detection

**v1.0**:
- Professional security audit
- 2FA implementation
- Compliance certifications

---

**Remember**: Security is an ongoing process, not a destination. Stay vigilant! 🔐
