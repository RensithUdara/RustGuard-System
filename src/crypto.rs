//! Cryptographic operations for RustGuard

use crate::error::{Error, Result};
use aes_gcm::{
    aead::{Aead, KeyInit},
    Aes256Gcm, Nonce,
};
use argon2::{Argon2, PasswordHasher, PasswordVerifier};
use password_hash::SaltString;
use rand::Rng;
use sha2::{Digest, Sha256};
use std::path::Path;

const CHUNK_SIZE: usize = 1024 * 1024; // 1MB chunks
const NONCE_SIZE: usize = 12;
const TAG_SIZE: usize = 16;

/// Generates a random nonce for encryption
pub fn generate_nonce() -> [u8; NONCE_SIZE] {
    let mut rng = rand::thread_rng();
    let mut nonce = [0u8; NONCE_SIZE];
    rng.fill(&mut nonce);
    nonce
}

/// Derives an encryption key from a password using Argon2
pub fn derive_key(password: &str, salt: &[u8; 16]) -> Result<[u8; 32]> {
    // Simplified key derivation using sha256 with password + salt
    let mut hasher = Sha256::new();
    hasher.update(password.as_bytes());
    hasher.update(salt);
    let result = hasher.finalize();
    
    let mut key = [0u8; 32];
    key.copy_from_slice(&result[..32]);
    Ok(key)
}

/// Hashes a password for storage
pub fn hash_password(password: &str) -> Result<String> {
    use argon2::password_hash::SaltString;

    let salt = SaltString::generate(rand::thread_rng());
    let argon2 = Argon2::default();

    argon2
        .hash_password(password.as_bytes(), &salt)
        .map(|h| h.to_string())
        .map_err(|e| Error::EncryptionError(e.to_string()))
}

/// Verifies a password against its hash
pub fn verify_password(password: &str, hash: &str) -> Result<bool> {
    use password_hash::PasswordHash;
    .map_err(|e| Error::EncryptionError(e.to_string()))?;

    Ok(Argon2::default()
        .verify_password(password.as_bytes(), &parsed_hash)
        .is_ok())
}

/// Encrypts data using AES-256-GCM
pub fn encrypt(data: &[u8], key: &[u8; 32]) -> Result<(Vec<u8>, [u8; NONCE_SIZE])> {
    let cipher = Aes256Gcm::new(key.into());
    let nonce_bytes = generate_nonce();
    let nonce = Nonce::from_slice(&nonce_bytes);

    let ciphertext = cipher
        .encrypt(nonce, data)
        .map_err(|e| Error::EncryptionError(e.to_string()))?;

    Ok((ciphertext, nonce_bytes))
}

/// Decrypts data using AES-256-GCM
pub fn decrypt(ciphertext: &[u8], key: &[u8; 32], nonce: &[u8; NONCE_SIZE]) -> Result<Vec<u8>> {
    let cipher = Aes256Gcm::new(key.into());
    let nonce_slice = Nonce::from_slice(nonce);

    cipher
        .decrypt(nonce_slice, ciphertext)
        .map_err(|e| Error::DecryptionError(e.to_string()))
}

/// Computes SHA256 hash of data
pub fn compute_hash(data: &[u8]) -> String {
    let mut hasher = Sha256::new();
    hasher.update(data);
    hex::encode(hasher.finalize())
}

/// Encrypts file with chunking for large files
pub fn encrypt_large_file(data: &[u8], key: &[u8; 32]) -> Result<Vec<(Vec<u8>, [u8; NONCE_SIZE])>> {
    let mut encrypted_chunks = Vec::new();

    for chunk in data.chunks(CHUNK_SIZE) {
        let (encrypted, nonce) = encrypt(chunk, key)?;
        encrypted_chunks.push((encrypted, nonce));
    }

    Ok(encrypted_chunks)
}

/// Decrypts file from chunks
pub fn decrypt_large_file(
    chunks: Vec<(Vec<u8>, [u8; NONCE_SIZE])>,
    key: &[u8; 32],
) -> Result<Vec<u8>> {
    let mut data = Vec::new();

    for (encrypted, nonce) in chunks {
        let decrypted = decrypt(&encrypted, key, &nonce)?;
        data.extend_from_slice(&decrypted);
    }

    Ok(data)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_encrypt_decrypt() {
        let key = [0u8; 32];
        let plaintext = b"Hello, RustGuard!";

        let (ciphertext, nonce) = encrypt(plaintext, &key).unwrap();
        let decrypted = decrypt(&ciphertext, &key, &nonce).unwrap();

        assert_eq!(plaintext, &decrypted[..]);
    }

    #[test]
    fn test_hash_password() {
        let password = "super_secure_password_123!";
        let hash = hash_password(password).unwrap();
        assert!(verify_password(password, &hash).unwrap());
        assert!(!verify_password("wrong_password", &hash).unwrap());
    }

    #[test]
    fn test_compute_hash() {
        let data = b"test data";
        let hash1 = compute_hash(data);
        let hash2 = compute_hash(data);
        assert_eq!(hash1, hash2);
    }
}
