use mongodb::bson::DateTime;
use serde::{Deserialize, Serialize};

use super::my_date_format::my_date_format;

#[derive(Debug, Serialize, Deserialize)]
pub struct HRTS {
    #[serde(with = "my_date_format")]
    pub timestamp: DateTime,
    pub hr: f64,
}
