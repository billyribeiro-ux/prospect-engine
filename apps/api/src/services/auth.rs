use std::time::{Duration, SystemTime, UNIX_EPOCH};

use argon2::password_hash::{PasswordHash, PasswordHasher, PasswordVerifier, SaltString};
use argon2::Argon2;
use jsonwebtoken::{decode, encode, Algorithm, DecodingKey, EncodingKey, Header, Validation};
use rand::rngs::OsRng;
use serde::{Deserialize, Serialize};
use sqlx::SqlitePool;
use uuid::Uuid;

use crate::errors::ApiError;

const JWT_TTL_SECS: u64 = 60 * 60 * 24 * 7;
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

pub fn issue_token(jwt_secret: &str, user_id: &str, email: &str) -> Result<String, ApiError> {
    let exp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map_err(|_| ApiError::Internal)?
        + Duration::from_secs(JWT_TTL_SECS);
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
    pool: &SqlitePool,
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
    pool: &SqlitePool,
    email: &str,
) -> Result<Option<(String, String)>, ApiError> {
    let row: Option<(String, String)> = sqlx::query_as(
        "SELECT id, password_hash FROM users WHERE email = ? COLLATE NOCASE LIMIT 1",
    )
    .bind(email)
    .fetch_optional(pool)
    .await
    .map_err(|e| {
        tracing::error!(error = %e, "find user");
        ApiError::Internal
    })?;
    Ok(row)
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
        let token = issue_token(secret, "user-id", "person@example.com").expect("issue");
        let claims = verify_token(secret, &token).expect("verify");
        assert_eq!(claims.sub, "user-id");
        assert_eq!(claims.email, "person@example.com");
    }

    #[test]
    fn jwt_rejects_tampered_token() {
        let secret = "unit-test-jwt-secret-min-32-chars!!";
        let token = issue_token(secret, "uid", "a@b.com").expect("issue");
        let mut bad = token.clone();
        bad.pop();
        assert!(verify_token(secret, &bad).is_err());
    }
}
