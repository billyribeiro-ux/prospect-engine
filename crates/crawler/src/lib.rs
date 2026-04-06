#![deny(clippy::all, clippy::pedantic)]
#![allow(clippy::missing_errors_doc)]

pub mod http;

use thiserror::Error;

#[derive(Debug, Error)]
pub enum CrawlError {
    #[error("fetch failed: {0}")]
    Fetch(String),
}
