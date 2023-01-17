use mongodb::bson::oid::ObjectId;
use serde::{Deserialize, Serialize};

use super::{GPSTS_model::GPSTS, HRTS_model::HRTS};

/*#[derive(Debug, Serialize, Deserialize)]
enum TSData {
    GPSTS,
    HRTS,
}*/

#[derive(Debug, Serialize, Deserialize)]
pub struct Activity {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,
    pub length: f64,
    pub activity_title: String,
    //time series data:
    pub gps: Option<Vec<GPSTS>>,
    pub hr: Option<Vec<HRTS>>,
}

/*

time_series_data{
    [GPSTS, GPSTS, GPSTS],
    [HRTS, HRTS, HRTS],
}



*/
