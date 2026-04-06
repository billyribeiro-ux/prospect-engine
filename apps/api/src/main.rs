#![deny(clippy::all, clippy::pedantic)]
#![allow(clippy::missing_errors_doc)]
#![allow(clippy::missing_panics_doc)]

mod config;
mod errors;
mod extractors;
mod handlers;
mod middleware;
mod router;
mod services;

use std::net::SocketAddr;

use tower_http::cors::CorsLayer;
use tower_http::trace::TraceLayer;
use tracing_subscriber::EnvFilter;

#[derive(Clone)]
pub struct AppState {}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
	tracing_subscriber::fmt()
		.with_env_filter(EnvFilter::from_default_env())
		.init();

	let cfg = config::load()?;
	let state = AppState {};

	let app = router::api_router(state)
		.layer(CorsLayer::permissive())
		.layer(TraceLayer::new_for_http());

	let addr: SocketAddr = format!("{}:{}", cfg.bind_host, cfg.bind_port).parse()?;
	tracing::info!(%addr, "listening");
	let listener = tokio::net::TcpListener::bind(addr).await?;
	axum::serve(listener, app).await?;
	Ok(())
}
