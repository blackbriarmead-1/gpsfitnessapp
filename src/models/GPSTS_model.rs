use mongodb::bson::DateTime;
use serde::{Deserialize, Serialize};

use super::my_date_format::my_date_format;

#[derive(Debug, Serialize, Deserialize)]
pub struct GPSTS {
    #[serde(with = "my_date_format")]
    pub timestamp: DateTime,
    pub lat: f64,
    pub lon: f64,
}
