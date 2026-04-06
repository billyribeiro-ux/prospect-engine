use std::env;
use std::io;

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

    Ok(AppConfig {
        bind_host,
        bind_port,
        database_url,
        jwt_secret,
        cors_allow_origins,
    })
}

impl AppConfig {
    /// Builds a minimal config for integration tests (permissive CORS when origins are empty).
    #[must_use]
    pub fn for_tests(jwt_secret: &str) -> Self {
        Self {
            bind_host: "127.0.0.1".to_string(),
            bind_port: 8080,
            database_url: "sqlite::memory:?cache=shared".to_string(),
            jwt_secret: jwt_secret.to_string(),
            cors_allow_origins: vec![],
        }
    }
}
