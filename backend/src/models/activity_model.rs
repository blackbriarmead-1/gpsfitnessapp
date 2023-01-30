use mongodb::bson::oid::ObjectId;
use serde::{Deserialize, Serialize};

use super::{gpsts_model::GPSTS, hrts_model::HRTS};

/// Activity is the standard struct for representing activities.
///
/// It contains metadata related to an activity,
/// as well as time-series sensor data
#[derive(Debug, Serialize, Deserialize)]
pub struct Activity {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,
    pub distance: f64,
    pub activity_title: String,
    pub start_time: String,
    pub end_time: String,
    //time series data:
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gps: Option<Vec<GPSTS>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hr: Option<Vec<HRTS>>,
}
