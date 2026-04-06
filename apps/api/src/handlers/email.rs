use std::str::FromStr;

use axum::extract::State;
use axum::http::StatusCode;
use axum::Json;
use lettre::message::{Mailbox, Message};
use lettre::transport::smtp::authentication::Credentials;
use lettre::{SmtpTransport, Transport};
use serde::Deserialize;
use serde_json::json;
use uuid::Uuid;

use crate::errors::ApiError;
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

/// Accepts a send request; delivers via SMTP when `PE_SMTP_*` is configured (see `docs/SECURITY.md`).
#[allow(clippy::too_many_lines)]
pub async fn post_send(
    State(state): State<AppState>,
    Json(body): Json<SendEmailBody>,
) -> Result<(StatusCode, Json<serde_json::Value>), ApiError> {
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

    let Some(smtp) = state.smtp.clone() else {
        sqlx::query(
            "INSERT INTO email_events (id, recipient, subject, body_preview, status, detail, created_at) \
             VALUES (?, ?, ?, ?, 'stub', ?, ?)",
        )
        .bind(&id)
        .bind(to)
        .bind(&body.subject)
        .bind(&preview_text)
        .bind(Some(
            "PE_SMTP_HOST not set; message recorded only (no relay).".to_string(),
        ))
        .bind(&now)
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
        "INSERT INTO email_events (id, recipient, subject, body_preview, status, detail, created_at) \
         VALUES (?, ?, ?, ?, ?, ?, ?)",
    )
    .bind(&id)
    .bind(to)
    .bind(&body.subject)
    .bind(&preview_text)
    .bind(status)
    .bind(&detail)
    .bind(&now)
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
        })),
    ))
}
