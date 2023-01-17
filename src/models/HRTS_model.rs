use mongodb::bson::DateTime;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct HRTS {
    pub timestamp: DateTime,
    pub hr: f64,
}
