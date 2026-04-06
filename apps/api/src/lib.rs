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

use axum::extract::DefaultBodyLimit;
use sqlx::sqlite::SqlitePoolOptions;
use tower_http::cors::CorsLayer;
use tower_http::trace::TraceLayer;
use tracing_subscriber::EnvFilter;

/// Maximum JSON body size for `/api/v1/*` (auth and future POST endpoints).
const MAX_JSON_BODY_BYTES: usize = 32 * 1024;

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
        jwt_secret: cfg.jwt_secret,
    };

    let app = router::api_router(state)
        .layer(DefaultBodyLimit::max(MAX_JSON_BODY_BYTES))
        .layer(CorsLayer::permissive())
        .layer(TraceLayer::new_for_http());

    let addr: SocketAddr = format!("{}:{}", cfg.bind_host, cfg.bind_port).parse()?;
    tracing::info!(%addr, "listening");
    let listener = tokio::net::TcpListener::bind(addr).await?;
    axum::serve(listener, app).await?;
    Ok(())
}
