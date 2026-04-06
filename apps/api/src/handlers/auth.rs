use axum::extract::State;
use axum::http::HeaderMap;
use axum::Json;
use serde::Deserialize;

use crate::errors::ApiError;
use crate::models::{AuthSuccess, AuthUser, SessionResponse};
use crate::services::auth;
use crate::AppState;

/// Maximum password length (bytes) to bound Argon2 work for a single request.
const MAX_PASSWORD_BYTES: usize = 256;

#[derive(Deserialize)]
pub struct AuthBody {
    pub email: String,
    pub password: String,
}

fn normalize_email(s: &str) -> String {
    s.trim().to_lowercase()
}

fn bearer(headers: &HeaderMap) -> Option<String> {
    let h = headers
        .get(axum::http::header::AUTHORIZATION)?
        .to_str()
        .ok()?;
    let token = h.strip_prefix("Bearer ")?;
    let trimmed = token.trim();
    if trimmed.is_empty() {
        return None;
    }
    Some(trimmed.to_string())
}

fn validate_email(email: &str) -> Result<(), ApiError> {
    if !email_address::EmailAddress::is_valid(email) {
        return Err(ApiError::Validation("invalid email".into()));
    }
    Ok(())
}

fn validate_password(password: &str) -> Result<(), ApiError> {
    if password.len() < 8 {
        return Err(ApiError::Validation(
            "password must be at least 8 characters".into(),
        ));
    }
    if password.len() > MAX_PASSWORD_BYTES {
        return Err(ApiError::Validation("password is too long".into()));
    }
    Ok(())
}

pub async fn get_session(
    State(state): State<AppState>,
    headers: HeaderMap,
) -> Result<Json<SessionResponse>, ApiError> {
    if let Some(token) = bearer(&headers) {
        if let Ok(claims) = auth::verify_token(&state.jwt_secret, &token) {
            return Ok(Json(SessionResponse::authenticated(AuthUser {
                id: claims.sub,
                email: claims.email,
            })));
        }
    }
    Ok(Json(SessionResponse::anonymous()))
}

pub async fn post_register(
    State(state): State<AppState>,
    Json(body): Json<AuthBody>,
) -> Result<Json<AuthSuccess>, ApiError> {
    let email = normalize_email(&body.email);
    validate_email(&email)?;
    validate_password(&body.password)?;

    let hash = auth::hash_password(&body.password)?;
    let user_id = auth::insert_user(&state.pool, &email, &hash).await?;
    let token = auth::issue_token(&state.jwt_secret, &user_id, &email)?;
    Ok(Json(AuthSuccess {
        token,
        user: AuthUser { id: user_id, email },
    }))
}

pub async fn post_login(
    State(state): State<AppState>,
    Json(body): Json<AuthBody>,
) -> Result<Json<AuthSuccess>, ApiError> {
    let email = normalize_email(&body.email);
    validate_password(&body.password)?;

    let Some((id, hash)) = auth::find_user_by_email(&state.pool, &email).await? else {
        return Err(ApiError::Unauthorized);
    };
    if !auth::verify_password(&body.password, &hash)? {
        return Err(ApiError::Unauthorized);
    }
    let token = auth::issue_token(&state.jwt_secret, &id, &email)?;
    Ok(Json(AuthSuccess {
        token,
        user: AuthUser { id, email },
    }))
}
