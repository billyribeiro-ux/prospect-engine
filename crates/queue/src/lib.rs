#![deny(clippy::all, clippy::pedantic)]
#![allow(clippy::missing_errors_doc)]

use async_trait::async_trait;

#[async_trait]
pub trait JobQueue: Send + Sync {
	async fn enqueue(&self, job_id: &str) -> Result<(), std::io::Error>;
}
