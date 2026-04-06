//! Persisted SMTP settings (`smtp_settings`) and resolution vs environment `SmtpConfig`.

use sqlx::AnyPool;

use crate::config::SmtpConfig;
use crate::errors::ApiError;

use super::smtp_crypto;

#[derive(Debug)]
pub struct SmtpSettingsRow {
    pub enabled: bool,
    pub host: String,
    pub port: u16,
    pub username: Option<String>,
    pub password_encrypted: Option<String>,
    pub from_address: String,
}

#[derive(Debug, sqlx::FromRow)]
struct SmtpSettingsSqlRow {
    enabled: i64,
    host: String,
    port: i64,
    username: Option<String>,
    password_encrypted: Option<String>,
    from_address: String,
}

fn row_from_sql(row: SmtpSettingsSqlRow) -> SmtpSettingsRow {
    let p = row.port.clamp(1, i64::from(u16::MAX));
    let port = u16::try_from(p).unwrap_or(587);
    SmtpSettingsRow {
        enabled: row.enabled != 0,
        host: row.host,
        port,
        username: row.username,
        password_encrypted: row.password_encrypted,
        from_address: row.from_address,
    }
}

pub async fn fetch_row(pool: &AnyPool) -> Result<Option<SmtpSettingsRow>, ApiError> {
    let row: Option<SmtpSettingsSqlRow> = sqlx::query_as(
        "SELECT enabled, host, port, username, password_encrypted, from_address \
         FROM smtp_settings WHERE id = 1",
    )
    .fetch_optional(pool)
    .await
    .map_err(|e| {
        tracing::error!(error = %e, "smtp_settings fetch");
        ApiError::Internal
    })?;
    Ok(row.map(row_from_sql))
}

/// When the saved row is enabled and has a non-empty host, build [`SmtpConfig`] (decrypt password).
pub async fn load_active_from_database(
    pool: &AnyPool,
    jwt_secret: &str,
) -> Option<SmtpConfig> {
    let row = fetch_row(pool).await.ok()??;
    if !row.enabled {
        return None;
    }
    let host = row.host.trim();
    if host.is_empty() {
        return None;
    }
    let password = row
        .password_encrypted
        .as_deref()
        .and_then(|e| smtp_crypto::decrypt_password(jwt_secret, e));
    Some(SmtpConfig {
        host: host.to_string(),
        port: row.port,
        username: row.username.clone(),
        password,
        from: row.from_address.clone(),
    })
}

/// Saved settings take precedence when enabled with a non-empty host; otherwise `env_smtp`.
pub async fn resolve_effective_smtp(
    pool: &AnyPool,
    jwt_secret: &str,
    env_smtp: Option<SmtpConfig>,
) -> Option<SmtpConfig> {
    if let Some(cfg) = load_active_from_database(pool, jwt_secret).await {
        return Some(cfg);
    }
    env_smtp
}

#[derive(Debug)]
pub struct UpsertInput<'a> {
    pub enabled: bool,
    pub host: &'a str,
    pub port: u16,
    pub username: Option<&'a str>,
    /// `Some` when caller wants a new password; `None` to keep existing (via SQL `COALESCE`).
    pub password_plain: Option<&'a str>,
    pub from_address: &'a str,
}

pub async fn upsert(pool: &AnyPool, jwt_secret: &str, input: UpsertInput<'_>) -> Result<(), ApiError> {
    let password_enc_new: Option<String> = match input.password_plain {
        Some(s) if !s.is_empty() => Some(smtp_crypto::encrypt_password(jwt_secret, s)?),
        _ => None,
    };

    let now = chrono::Utc::now().to_rfc3339();
    let username_owned = input.username.map(str::to_string);

    sqlx::query(
        "INSERT INTO smtp_settings (id, enabled, host, port, username, password_encrypted, from_address, updated_at) \
         VALUES (1, ?, ?, ?, ?, ?, ?, ?) \
         ON CONFLICT(id) DO UPDATE SET \
           enabled = excluded.enabled, \
           host = excluded.host, \
           port = excluded.port, \
           username = excluded.username, \
           from_address = excluded.from_address, \
           updated_at = excluded.updated_at, \
           password_encrypted = COALESCE(excluded.password_encrypted, smtp_settings.password_encrypted)",
    )
    .bind(input.enabled)
    .bind(input.host)
    .bind(i64::from(input.port))
    .bind(username_owned)
    .bind(password_enc_new)
    .bind(input.from_address)
    .bind(now)
    .execute(pool)
    .await
    .map_err(|e| {
        tracing::error!(error = %e, "smtp_settings upsert");
        ApiError::Internal
    })?;

    Ok(())
}
