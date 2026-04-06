use thiserror::Error;

#[derive(Debug, Error)]
pub enum SharedError {
    #[error("invalid identifier: {0}")]
    InvalidId(String),
}
