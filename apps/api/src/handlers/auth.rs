use axum::extract::State;
use axum::http::HeaderMap;
use axum::Json;
use serde::Deserialize;
use serde_json::{json, Value};

use crate::errors::ApiError;
use crate::services::auth;
use crate::AppState;

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
    h.strip_prefix("Bearer ")
        .map(std::string::ToString::to_string)
}

pub async fn get_session(
    State(state): State<AppState>,
    headers: HeaderMap,
) -> Result<Json<Value>, ApiError> {
    if let Some(token) = bearer(&headers) {
        if let Ok(claims) = auth::verify_token(&state.jwt_secret, &token) {
            return Ok(Json(json!({
                "authenticated": true,
                "user": { "id": claims.sub, "email": claims.email }
            })));
        }
    }
    Ok(Json(json!({ "authenticated": false })))
}

pub async fn post_register(
    State(state): State<AppState>,
    Json(body): Json<AuthBody>,
) -> Result<Json<Value>, ApiError> {
    let email = normalize_email(&body.email);
    if email.is_empty() || !email.contains('@') {
        return Err(ApiError::Validation("invalid email".into()));
    }
    if body.password.len() < 8 {
        return Err(ApiError::Validation(
            "password must be at least 8 characters".into(),
        ));
    }
    let hash = auth::hash_password(&body.password)?;
    let user_id = auth::insert_user(&state.pool, &email, &hash).await?;
    let token = auth::issue_token(&state.jwt_secret, &user_id, &email)?;
    Ok(Json(json!({
        "token": token,
        "user": { "id": user_id, "email": email }
    })))
}

pub async fn post_login(
    State(state): State<AppState>,
    Json(body): Json<AuthBody>,
) -> Result<Json<Value>, ApiError> {
    let email = normalize_email(&body.email);
    let Some((id, hash)) = auth::find_user_by_email(&state.pool, &email).await? else {
        return Err(ApiError::Unauthorized);
    };
    if !auth::verify_password(&body.password, &hash)? {
        return Err(ApiError::Unauthorized);
    }
    let token = auth::issue_token(&state.jwt_secret, &id, &email)?;
    Ok(Json(json!({
        "token": token,
        "user": { "id": id, "email": email }
    })))
}
