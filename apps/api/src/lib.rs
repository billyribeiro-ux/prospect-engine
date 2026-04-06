#![deny(clippy::all, clippy::pedantic)]
#![allow(clippy::missing_errors_doc)]
#![allow(clippy::missing_panics_doc)]

pub mod config;
pub mod db_url;
pub mod errors;
pub mod extractors;
pub mod handlers;
pub mod middleware;
pub mod models;
pub mod router;
pub mod services;
pub mod state;

pub use state::AppState;

use std::net::SocketAddr;
use std::sync::Arc;

use queue::MemoryQueue;
use sqlx::any::{install_default_drivers, AnyPoolOptions};
use tracing_subscriber::EnvFilter;

/// Maximum JSON body size for `/api/v1/*` (auth and future POST endpoints).
pub(crate) const MAX_JSON_BODY_BYTES: usize = 32 * 1024;

/// Builds the full HTTP stack (CORS, security headers, request IDs, tracing) around the API router.
///
/// Use this in integration tests so behavior matches production. The binary [`run`] uses this internally.
pub fn build_http_app(state: AppState, cfg: &config::AppConfig) -> axum::Router {
    middleware::layers::apply_global_layers(router::api_router(state), cfg)
}

/// Boots tracing, `SQLite`, migrations, and the HTTP server. Intended to be invoked from `main`.
///
/// # Errors
///
/// Returns if configuration is invalid, the database cannot be opened, or the server fails to bind.
pub async fn run() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env())
        .init();

    let cfg = config::load()?;

    std::fs::create_dir_all("data")?;

    install_default_drivers();
    let db_url = db_url::normalize_database_url(&cfg.database_url);
    let pool = AnyPoolOptions::new()
        .max_connections(5)
        .connect(&db_url)
        .await?;

    sqlx::migrate!("./migrations").run(&pool).await?;

    let email_rate = Arc::new(crate::services::rate_limit::MinuteWindowLimiter::new(
        cfg.email_rate_limit_per_min as usize,
    ));

    let state = AppState {
        pool: pool.clone(),
        jwt_secret: cfg.jwt_secret.clone(),
        job_queue: Arc::new(MemoryQueue::new()),
        smtp: cfg.smtp.clone(),
        email_rate,
        public_api_origin: cfg.public_api_origin.clone(),
    };

    let pool_worker = pool.clone();
    tokio::spawn(async move {
        crate::services::worker::run_durable_worker(pool_worker).await;
    });

    let app = build_http_app(state, &cfg);

    let addr: SocketAddr = format!("{}:{}", cfg.bind_host, cfg.bind_port).parse()?;
    tracing::info!(%addr, "listening");
    let listener = tokio::net::TcpListener::bind(addr).await?;
    axum::serve(
        listener,
        app.into_make_service_with_connect_info::<SocketAddr>(),
    )
    .await?;
    Ok(())
}
