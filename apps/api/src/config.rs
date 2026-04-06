use std::env;
use std::io;

/// SMTP relay settings when `PE_SMTP_HOST` is set (optional).
#[derive(Debug, Clone)]
pub struct SmtpConfig {
    pub host: String,
    pub port: u16,
    pub username: Option<String>,
    pub password: Option<String>,
    pub from: String,
}

/// Runtime configuration loaded from environment variables.
#[derive(Debug, Clone)]
pub struct AppConfig {
    pub bind_host: String,
    pub bind_port: u16,
    pub database_url: String,
    pub jwt_secret: String,
    /// Allowed browser `Origin` values for CORS (comma-separated). Empty means permissive CORS
    /// in debug builds only; release builds require an explicit list.
    pub cors_allow_origins: Vec<String>,
    /// When `Some`, `POST /email/send` attempts delivery via SMTP.
    pub smtp: Option<SmtpConfig>,
    /// Max `POST /email/send` requests per client key per rolling minute (`0` = disabled).
    pub email_rate_limit_per_min: u32,
    /// Optional absolute origin for tracking URLs in JSON responses (e.g. `https://api.example.com`).
    pub public_api_origin: Option<String>,
}

fn load_smtp() -> Option<SmtpConfig> {
    let host = env::var("PE_SMTP_HOST").ok()?.trim().to_string();
    if host.is_empty() {
        return None;
    }
    let port = env::var("PE_SMTP_PORT")
        .ok()
        .and_then(|s| s.parse().ok())
        .unwrap_or(587);
    let username = env::var("PE_SMTP_USER")
        .ok()
        .filter(|s| !s.trim().is_empty());
    let password = env::var("PE_SMTP_PASSWORD").ok().filter(|s| !s.is_empty());
    let from = env::var("PE_SMTP_FROM").unwrap_or_else(|_| "noreply@localhost".to_string());
    Some(SmtpConfig {
        host,
        port,
        username,
        password,
        from,
    })
}

pub fn load() -> Result<AppConfig, io::Error> {
    let bind_host = env::var("PE_API_BIND_HOST").unwrap_or_else(|_| "127.0.0.1".to_string());
    let raw_port = env::var("PE_API_BIND_PORT").ok();
    let bind_port = match raw_port.as_deref() {
        None => 8080,
        Some(value) => value
            .parse()
            .map_err(|_| io::Error::new(io::ErrorKind::InvalidInput, "invalid PE_API_BIND_PORT"))?,
    };

    let database_url =
        env::var("PE_DATABASE_URL").unwrap_or_else(|_| "sqlite:data/pe.db".to_string());

    let jwt_secret = env::var("PE_JWT_SECRET").unwrap_or_else(|_| {
        if cfg!(debug_assertions) {
            tracing::warn!("PE_JWT_SECRET not set; using insecure development default");
            "dev-only-change-with-PE_JWT_SECRET-min-32-chars".to_string()
        } else {
            String::new()
        }
    });

    if jwt_secret.len() < 32 && !cfg!(debug_assertions) {
        return Err(io::Error::new(
            io::ErrorKind::InvalidInput,
            "PE_JWT_SECRET must be set to at least 32 characters in release builds",
        ));
    }

    let cors_allow_origins: Vec<String> = env::var("PE_CORS_ALLOW_ORIGINS")
        .map(|raw| {
            raw.split(',')
                .map(str::trim)
                .filter(|s| !s.is_empty())
                .map(String::from)
                .collect()
        })
        .unwrap_or_default();

    if cors_allow_origins.is_empty() {
        if cfg!(debug_assertions) {
            tracing::warn!(
                "PE_CORS_ALLOW_ORIGINS not set; using permissive CORS (not for production)"
            );
        } else {
            return Err(io::Error::new(
                io::ErrorKind::InvalidInput,
                "PE_CORS_ALLOW_ORIGINS must list at least one allowed origin in release builds \
                 (comma-separated, e.g. https://app.example.com)",
            ));
        }
    }

    let smtp = load_smtp();
    if smtp.is_some() {
        tracing::info!("PE_SMTP_HOST set: outbound email delivery enabled");
    }

    let email_rate_limit_per_min = env::var("PE_EMAIL_RATE_LIMIT_PER_MIN")
        .ok()
        .and_then(|s| s.parse().ok())
        .unwrap_or(20);

    let public_api_origin = env::var("PE_PUBLIC_API_ORIGIN")
        .ok()
        .map(|s| s.trim().to_string())
        .filter(|s| !s.is_empty());

    Ok(AppConfig {
        bind_host,
        bind_port,
        database_url,
        jwt_secret,
        cors_allow_origins,
        smtp,
        email_rate_limit_per_min,
        public_api_origin,
    })
}

impl AppConfig {
    /// Builds a minimal config for integration tests (permissive CORS when origins are empty).
    #[must_use]
    pub fn for_tests(jwt_secret: &str) -> Self {
        Self::for_tests_with_email_rate(jwt_secret, 0)
    }

    /// Test config with a specific email rate limit (mirrors [`AppState::for_tests_with_email_rate`](crate::state::AppState::for_tests_with_email_rate)).
    #[must_use]
    pub fn for_tests_with_email_rate(jwt_secret: &str, email_rate_per_min: u32) -> Self {
        Self {
            bind_host: "127.0.0.1".to_string(),
            bind_port: 8080,
            database_url: "sqlite::memory:?cache=shared".to_string(),
            jwt_secret: jwt_secret.to_string(),
            cors_allow_origins: vec![],
            smtp: None,
            email_rate_limit_per_min: email_rate_per_min,
            public_api_origin: None,
        }
    }
}
