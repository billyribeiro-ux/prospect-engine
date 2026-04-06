use std::time::{Duration, SystemTime, UNIX_EPOCH};

use argon2::password_hash::{PasswordHash, PasswordHasher, PasswordVerifier, SaltString};
use argon2::Argon2;
use jsonwebtoken::{decode, encode, Algorithm, DecodingKey, EncodingKey, Header, Validation};
use rand::rngs::OsRng;
use rand::RngCore;
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use sqlx::AnyPool;
use uuid::Uuid;

use crate::errors::ApiError;

/// Short-lived access JWT (Bearer).
pub const ACCESS_TOKEN_TTL_SECS: u64 = 60 * 15;
/// Refresh token lifetime (opaque bearer, stored hashed).
pub const REFRESH_TOKEN_TTL_SECS: u64 = 60 * 60 * 24 * 30;
/// Clock skew leeway (seconds) for `exp` validation.
const JWT_LEEWAY_SECS: u64 = 60;

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String,
    pub email: String,
    pub exp: u64,
}

pub fn hash_password(password: &str) -> Result<String, ApiError> {
    let salt = SaltString::generate(&mut OsRng);
    let argon2 = Argon2::default();
    let hash = argon2
        .hash_password(password.as_bytes(), &salt)
        .map_err(|_| ApiError::Internal)?;
    Ok(hash.to_string())
}

pub fn verify_password(password: &str, password_hash: &str) -> Result<bool, ApiError> {
    let parsed = PasswordHash::new(password_hash).map_err(|_| ApiError::Internal)?;
    Ok(Argon2::default()
        .verify_password(password.as_bytes(), &parsed)
        .is_ok())
}

fn hash_refresh_plain(plain: &str) -> String {
    let digest = Sha256::digest(plain.as_bytes());
    hex::encode(digest)
}

fn random_refresh_plain() -> String {
    let mut buf = [0u8; 32];
    OsRng.fill_bytes(&mut buf);
    hex::encode(buf)
}

pub fn issue_access_token(
    jwt_secret: &str,
    user_id: &str,
    email: &str,
) -> Result<String, ApiError> {
    let exp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map_err(|_| ApiError::Internal)?
        + Duration::from_secs(ACCESS_TOKEN_TTL_SECS);
    let exp_secs = exp.as_secs();
    let claims = Claims {
        sub: user_id.to_string(),
        email: email.to_string(),
        exp: exp_secs,
    };
    let header = Header::new(Algorithm::HS256);
    encode(
        &header,
        &claims,
        &EncodingKey::from_secret(jwt_secret.as_bytes()),
    )
    .map_err(|_| ApiError::Internal)
}

pub fn verify_token(jwt_secret: &str, token: &str) -> Result<Claims, ApiError> {
    let mut validation = Validation::new(Algorithm::HS256);
    validation.leeway = JWT_LEEWAY_SECS;
    let data = decode::<Claims>(
        token,
        &DecodingKey::from_secret(jwt_secret.as_bytes()),
        &validation,
    )
    .map_err(|_| ApiError::Unauthorized)?;
    let claims = data.claims;
    if claims.sub.is_empty() || claims.email.is_empty() {
        return Err(ApiError::Unauthorized);
    }
    Ok(claims)
}

pub async fn insert_user(
    pool: &AnyPool,
    email: &str,
    password_hash: &str,
) -> Result<String, ApiError> {
    let id = Uuid::new_v4().to_string();
    let now = chrono::Utc::now().to_rfc3339();
    let res =
        sqlx::query("INSERT INTO users (id, email, password_hash, created_at) VALUES (?, ?, ?, ?)")
            .bind(&id)
            .bind(email)
            .bind(password_hash)
            .bind(&now)
            .execute(pool)
            .await;
    match res {
        Ok(_) => Ok(id),
        Err(sqlx::Error::Database(db)) if db.is_unique_violation() => {
            Err(ApiError::Conflict("email already registered".to_string()))
        }
        Err(e) => {
            tracing::error!(error = %e, "insert user");
            Err(ApiError::Internal)
        }
    }
}

pub async fn find_user_by_email(
    pool: &AnyPool,
    email: &str,
) -> Result<Option<(String, String)>, ApiError> {
    // Email is stored normalized (lowercase) on insert; avoid `lower(?)` for Any+SQLite quirks.
    let row: Option<(String, String)> =
        sqlx::query_as("SELECT id, password_hash FROM users WHERE email = ? LIMIT 1")
            .bind(email)
            .fetch_optional(pool)
            .await
            .map_err(|e| {
                tracing::error!(error = %e, "find user");
                ApiError::Internal
            })?;
    Ok(row)
}

pub async fn find_user_email(pool: &AnyPool, user_id: &str) -> Result<Option<String>, ApiError> {
    let row: Option<(String,)> = sqlx::query_as("SELECT email FROM users WHERE id = ? LIMIT 1")
        .bind(user_id)
        .fetch_optional(pool)
        .await
        .map_err(|e| {
            tracing::error!(error = %e, "find user email");
            ApiError::Internal
        })?;
    Ok(row.map(|t| t.0))
}

async fn delete_refresh_for_user(pool: &AnyPool, user_id: &str) -> Result<(), ApiError> {
    sqlx::query("DELETE FROM refresh_tokens WHERE user_id = ?")
        .bind(user_id)
        .execute(pool)
        .await
        .map_err(|e| {
            tracing::error!(error = %e, "delete refresh tokens");
            ApiError::Internal
        })?;
    Ok(())
}

/// Stores a new refresh token (replaces any existing for this user).
pub async fn store_refresh_token(pool: &AnyPool, user_id: &str) -> Result<String, ApiError> {
    delete_refresh_for_user(pool, user_id).await?;
    let plain = random_refresh_plain();
    let token_hash = hash_refresh_plain(&plain);
    let id = Uuid::new_v4().to_string();
    let now = chrono::Utc::now();
    let expires = now
        + chrono::Duration::from_std(std::time::Duration::from_secs(REFRESH_TOKEN_TTL_SECS))
            .map_err(|_| ApiError::Internal)?;
    sqlx::query(
        "INSERT INTO refresh_tokens (id, user_id, token_hash, expires_at, created_at) VALUES (?, ?, ?, ?, ?)",
    )
    .bind(&id)
    .bind(user_id)
    .bind(&token_hash)
    .bind(expires.to_rfc3339())
    .bind(now.to_rfc3339())
    .execute(pool)
    .await
    .map_err(|e| {
        tracing::error!(error = %e, "insert refresh token");
        ApiError::Internal
    })?;
    Ok(plain)
}

/// Validates refresh token, removes it, and returns `user_id`.
pub async fn consume_refresh_token(pool: &AnyPool, plain: &str) -> Result<String, ApiError> {
    let token_hash = hash_refresh_plain(plain.trim());
    let now = chrono::Utc::now().to_rfc3339();
    let row: Option<(String, String)> = sqlx::query_as(
        "SELECT id, user_id FROM refresh_tokens WHERE token_hash = ? AND expires_at > ? LIMIT 1",
    )
    .bind(&token_hash)
    .bind(&now)
    .fetch_optional(pool)
    .await
    .map_err(|e| {
        tracing::error!(error = %e, "lookup refresh");
        ApiError::Internal
    })?;
    let Some((row_id, user_id)) = row else {
        return Err(ApiError::Unauthorized);
    };
    sqlx::query("DELETE FROM refresh_tokens WHERE id = ?")
        .bind(&row_id)
        .execute(pool)
        .await
        .map_err(|_| ApiError::Internal)?;
    Ok(user_id)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn hash_and_verify_roundtrip() {
        let hash = hash_password("correct-horse-battery-staple-unique").expect("hash");
        assert!(verify_password("correct-horse-battery-staple-unique", &hash).expect("verify"));
        assert!(!verify_password("wrong-password", &hash).expect("verify bool"));
    }

    #[test]
    fn jwt_roundtrip_and_claims() {
        let secret = "unit-test-jwt-secret-min-32-chars!!";
        let token = issue_access_token(secret, "user-id", "person@example.com").expect("issue");
        let claims = verify_token(secret, &token).expect("verify");
        assert_eq!(claims.sub, "user-id");
        assert_eq!(claims.email, "person@example.com");
    }

    #[test]
    fn jwt_rejects_tampered_token() {
        let secret = "unit-test-jwt-secret-min-32-chars!!";
        let token = issue_access_token(secret, "uid", "a@b.com").expect("issue");
        let mut bad = token.clone();
        bad.pop();
        assert!(verify_token(secret, &bad).is_err());
    }
}
