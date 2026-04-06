use std::str::FromStr;

use axum::extract::State;
use axum::http::HeaderMap;
use axum::Json;
use lettre::message::Mailbox;
use serde::{Deserialize, Serialize};

use crate::config::SmtpConfig;
use crate::errors::ApiError;
use crate::services::auth;
use crate::services::smtp_settings;
use crate::AppState;

use super::auth::bearer_token;

#[derive(Serialize)]
pub struct EnvironmentSmtpInfo {
    pub configured: bool,
    pub host: Option<String>,
    pub port: Option<u16>,
}

#[derive(Serialize)]
pub struct SmtpSettingsResponse {
    pub enabled: bool,
    pub host: String,
    pub port: u16,
    pub username: Option<String>,
    pub from: String,
    pub has_password: bool,
    pub active_source: &'static str,
    pub environment: EnvironmentSmtpInfo,
}

#[derive(Deserialize)]
pub struct PutSmtpBody {
    pub enabled: bool,
    pub host: String,
    pub port: u16,
    #[serde(default)]
    pub username: Option<String>,
    /// When set to a non-empty string, replaces the stored password; omit or empty keeps existing.
    #[serde(default)]
    pub password: Option<String>,
    pub from: String,
}

fn active_source(row: Option<&smtp_settings::SmtpSettingsRow>, env: Option<&SmtpConfig>) -> &'static str {
    if let Some(r) = row {
        if r.enabled && !r.host.trim().is_empty() {
            return "database";
        }
    }
    if env.is_some() {
        return "environment";
    }
    "none"
}

async fn json_response(state: &AppState) -> Result<Json<SmtpSettingsResponse>, ApiError> {
    let row = smtp_settings::fetch_row(&state.pool).await?;
    let env = state.smtp.as_ref();

    let (enabled, host, port, username, from, has_password) = match &row {
        Some(r) => (
            r.enabled,
            r.host.clone(),
            r.port,
            r.username.clone(),
            r.from_address.clone(),
            r.password_encrypted.as_ref().is_some_and(|s| !s.is_empty()),
        ),
        None => (false, String::new(), 587, None, String::new(), false),
    };

    let environment = EnvironmentSmtpInfo {
        configured: env.is_some(),
        host: env.map(|e| e.host.clone()),
        port: env.map(|e| e.port),
    };

    Ok(Json(SmtpSettingsResponse {
        enabled,
        host,
        port,
        username,
        from,
        has_password,
        active_source: active_source(row.as_ref(), env),
        environment,
    }))
}

pub async fn get_smtp(
    State(state): State<AppState>,
    headers: HeaderMap,
) -> Result<Json<SmtpSettingsResponse>, ApiError> {
    let token = bearer_token(&headers).ok_or(ApiError::Unauthorized)?;
    let _claims = auth::verify_token(&state.jwt_secret, &token)?;
    json_response(&state).await
}

pub async fn put_smtp(
    State(state): State<AppState>,
    headers: HeaderMap,
    Json(body): Json<PutSmtpBody>,
) -> Result<Json<SmtpSettingsResponse>, ApiError> {
    let token = bearer_token(&headers).ok_or(ApiError::Unauthorized)?;
    let _claims = auth::verify_token(&state.jwt_secret, &token)?;

    let host = body.host.trim();
    let from = body.from.trim();
    if body.enabled {
        if host.is_empty() {
            return Err(ApiError::Validation(
                "SMTP host is required when SMTP is enabled".into(),
            ));
        }
        if from.is_empty() {
            return Err(ApiError::Validation(
                "from address is required when SMTP is enabled".into(),
            ));
        }
        Mailbox::from_str(from).map_err(|_| {
            ApiError::Validation("invalid from address (use Name <email@domain> or email@domain)".into())
        })?;
        if !(1..=65535).contains(&body.port) {
            return Err(ApiError::Validation("port must be between 1 and 65535".into()));
        }
    }

    let username = body
        .username
        .as_deref()
        .map(str::trim)
        .filter(|s| !s.is_empty());

    let password_plain = body.password.as_deref().and_then(|s| {
        let t = s.trim();
        if t.is_empty() {
            None
        } else {
            Some(t)
        }
    });

    smtp_settings::upsert(
        &state.pool,
        &state.jwt_secret,
        smtp_settings::UpsertInput {
            enabled: body.enabled,
            host,
            port: body.port,
            username,
            password_plain,
            from_address: if from.is_empty() { "" } else { from },
        },
    )
    .await?;

    json_response(&state).await
}
