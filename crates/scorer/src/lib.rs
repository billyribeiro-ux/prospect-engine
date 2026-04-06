#![deny(clippy::all, clippy::pedantic)]

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CompositeScore {
    pub value: u8,
}
