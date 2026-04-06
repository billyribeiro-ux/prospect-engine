use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TenantRow {
    pub id: String,
    pub name: String,
}
