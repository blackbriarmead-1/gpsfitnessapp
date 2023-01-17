use mongodb::bson::DateTime;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct GPSTS {
    pub timestamp: DateTime,
    pub lat: f64,
    pub lon: f64,
}
