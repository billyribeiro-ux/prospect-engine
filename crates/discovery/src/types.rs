use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DiscoveryQuery {
    pub latitude: f64,
    pub longitude: f64,
    pub radius_meters: f64,
}
