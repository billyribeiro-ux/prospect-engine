#![deny(clippy::all, clippy::pedantic)]
#![allow(clippy::missing_errors_doc)]
#![allow(clippy::missing_panics_doc)]

pub mod config;
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
use sqlx::sqlite::SqlitePoolOptions;
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

    let pool = SqlitePoolOptions::new()
        .max_connections(5)
        .connect(&cfg.database_url)
        .await?;

    sqlx::migrate!("./migrations").run(&pool).await?;

    let state = AppState {
        pool,
        jwt_secret: cfg.jwt_secret.clone(),
        job_queue: Arc::new(MemoryQueue::new()),
    };

    let app = build_http_app(state, &cfg);

    let addr: SocketAddr = format!("{}:{}", cfg.bind_host, cfg.bind_port).parse()?;
    tracing::info!(%addr, "listening");
    let listener = tokio::net::TcpListener::bind(addr).await?;
    axum::serve(listener, app).await?;
    Ok(())
}
