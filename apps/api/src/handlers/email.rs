use std::str::FromStr;

use axum::extract::State;
use axum::http::{HeaderMap, StatusCode};
use axum::Json;
use lettre::message::{Mailbox, Message};
use lettre::transport::smtp::authentication::Credentials;
use lettre::{SmtpTransport, Transport};
use rand::RngCore;
use serde::Deserialize;
use serde_json::json;
use uuid::Uuid;

use crate::errors::ApiError;
use crate::services::smtp_settings;
use crate::AppState;

#[derive(Deserialize)]
pub struct SendEmailBody {
    pub to: String,
    pub subject: String,
    pub body: String,
}

fn preview(s: &str) -> String {
    s.chars().take(500).collect()
}

fn client_key(headers: &HeaderMap) -> String {
    headers
        .get("x-forwarded-for")
        .and_then(|v| v.to_str().ok())
        .and_then(|s| s.split(',').next().map(str::trim))
        .map_or_else(|| "direct".to_string(), String::from)
}

fn gen_tracking_token() -> String {
    let mut b = [0u8; 16];
    rand::thread_rng().fill_bytes(&mut b);
    hex::encode(b)
}

fn tracking_urls(state: &AppState, token: &str) -> serde_json::Value {
    let open = format!("/api/v1/email/track/open/{token}");
    let click = format!("/api/v1/email/track/click/{token}");
    match &state.public_api_origin {
        Some(o) => {
            let base = o.trim_end_matches('/');
            json!({
                "openPixelUrl": format!("{base}{open}"),
                "clickUrlTemplate": format!("{base}{click}?u="),
            })
        }
        None => json!({
            "openPixelUrl": open,
            "clickUrlTemplate": format!("{click}?u="),
        }),
    }
}

/// Accepts a send request; delivers via SMTP when saved Settings SMTP or `PE_SMTP_*` is active.
#[allow(clippy::too_many_lines)]
pub async fn post_send(
    State(state): State<AppState>,
    headers: HeaderMap,
    Json(body): Json<SendEmailBody>,
) -> Result<(StatusCode, Json<serde_json::Value>), ApiError> {
    state.email_rate.check(&client_key(&headers)).await?;

    let to = body.to.trim();
    if to.is_empty() {
        return Err(ApiError::Validation("to is required".into()));
    }
    if !email_address::EmailAddress::is_valid(to) {
        return Err(ApiError::Validation("invalid recipient email".into()));
    }

    let id = Uuid::new_v4().to_string();
    let now = chrono::Utc::now().to_rfc3339();
    let preview_text = preview(&body.body);
    let tracking_token = gen_tracking_token();

    let smtp = smtp_settings::resolve_effective_smtp(
        &state.pool,
        &state.jwt_secret,
        state.smtp.clone(),
    )
    .await;

    let Some(smtp) = smtp else {
        sqlx::query(
            "INSERT INTO email_events (id, recipient, subject, body_preview, status, detail, created_at, tracking_token, opens, clicks) \
             VALUES (?, ?, ?, ?, 'stub', ?, ?, ?, 0, 0)",
        )
        .bind(&id)
        .bind(to)
        .bind(&body.subject)
        .bind(&preview_text)
        .bind(Some(
            "No SMTP relay configured. Add SMTP in Settings or set PE_SMTP_* environment variables."
                .to_string(),
        ))
        .bind(&now)
        .bind(&tracking_token)
        .execute(&state.pool)
        .await
        .map_err(|e| {
            tracing::error!(error = %e, "email_events insert");
            ApiError::Internal
        })?;
        return Ok((
            StatusCode::ACCEPTED,
            Json(json!({
                "status": "accepted",
                "mode": "stub",
                "id": id,
                "to": to,
                "subject": body.subject,
                "tracking": tracking_urls(&state, &tracking_token),
            })),
        ));
    };

    let from = smtp.from.clone();
    let host = smtp.host.clone();
    let port = smtp.port;
    let user = smtp.username.clone();
    let pass = smtp.password.clone();
    let subject = body.subject.clone();
    let text = body.body.clone();
    let recipient = to.to_string();

    let send_result = tokio::task::spawn_blocking(move || {
        let from_mb = Mailbox::from_str(&from).map_err(|e| e.to_string())?;
        let to_mb = Mailbox::from_str(&recipient).map_err(|e| e.to_string())?;
        let email = Message::builder()
            .from(from_mb)
            .to(to_mb)
            .subject(subject)
            .body(text)
            .map_err(|e| e.to_string())?;

        let mut builder = SmtpTransport::relay(&host).map_err(|e| e.to_string())?;
        builder = builder.port(port);
        if let (Some(u), Some(p)) = (user, pass) {
            builder = builder.credentials(Credentials::new(u, p));
        }
        let transport = builder.build();
        transport.send(&email).map_err(|e| e.to_string())
    })
    .await;

    let (status, detail): (&str, Option<String>) = match send_result {
        Ok(Ok(_response)) => ("sent", None),
        Ok(Err(e)) => {
            tracing::warn!(error = %e, "smtp send failed");
            ("failed", Some(e))
        }
        Err(e) => {
            tracing::error!(error = %e, "smtp task join");
            ("failed", Some(e.to_string()))
        }
    };

    sqlx::query(
        "INSERT INTO email_events (id, recipient, subject, body_preview, status, detail, created_at, tracking_token, opens, clicks) \
         VALUES (?, ?, ?, ?, ?, ?, ?, ?, 0, 0)",
    )
    .bind(&id)
    .bind(to)
    .bind(&body.subject)
    .bind(&preview_text)
    .bind(status)
    .bind(&detail)
    .bind(&now)
    .bind(&tracking_token)
    .execute(&state.pool)
    .await
    .map_err(|e| {
        tracing::error!(error = %e, "email_events insert");
        ApiError::Internal
    })?;

    Ok((
        StatusCode::ACCEPTED,
        Json(json!({
            "status": "accepted",
            "mode": "smtp",
            "delivery": status,
            "id": id,
            "to": to,
            "subject": body.subject,
            "tracking": tracking_urls(&state, &tracking_token),
        })),
    ))
}
