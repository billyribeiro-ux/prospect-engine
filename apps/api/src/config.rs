use std::env;
use std::io;

#[derive(Debug, Clone)]
pub struct AppConfig {
    pub bind_host: String,
    pub bind_port: u16,
    pub database_url: String,
    pub jwt_secret: String,
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

    Ok(AppConfig {
        bind_host,
        bind_port,
        database_url,
        jwt_secret,
    })
}
