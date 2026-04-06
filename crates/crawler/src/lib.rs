#![deny(clippy::all, clippy::pedantic)]
#![allow(clippy::missing_errors_doc)]

pub mod http;

use thiserror::Error;

pub use http::fetch_url;

#[derive(Debug, Error)]
pub enum CrawlError {
    #[error("fetch failed: {0}")]
    Fetch(String),
}
