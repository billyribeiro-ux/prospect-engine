//! ProspectEngine Axum API binary entrypoint.

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    api::run().await
}
