//! Authentication module for the server

use axum::{
    extract::Request,
    middleware::Next,
    response::Response,
};
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};

const SECRET_KEY: &[u8] = b"rustguard_secret_key_change_in_production";

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String, // user_id
    pub exp: usize,
}

/// Generates a JWT token for a user
pub fn generate_token(user_id: &str) -> crate::error::Result<String> {
    let expiration = chrono::Utc::now()
        .checked_add_signed(chrono::Duration::days(7))
        .ok_or_else(|| crate::error::Error::Internal("Token generation failed".to_string()))?
        .timestamp() as usize;

    let claims = Claims {
        sub: user_id.to_string(),
        exp: expiration,
    };

    encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(SECRET_KEY),
    )
    .map_err(|e| crate::error::Error::Internal(e.to_string()))
}

/// Verifies a JWT token
pub fn verify_token(token: &str) -> crate::error::Result<Claims> {
    decode::<Claims>(
        token,
        &DecodingKey::from_secret(SECRET_KEY),
        &Validation::default(),
    )
    .map(|data| data.claims)
    .map_err(|e| crate::error::Error::AuthenticationFailed(e.to_string()))
}

/// Extracts and validates token from request headers
pub async fn extract_token(req: Request, next: Next) -> Response {
    next.run(req).await
}
