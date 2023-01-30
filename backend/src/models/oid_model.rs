use mongodb::bson::oid::ObjectId;
use serde::{Deserialize, Serialize};

/// OIDS is useful for representing a list of object ids for a request or response
///
#[derive(Debug, Serialize, Deserialize)]
pub struct OIDS {
    pub oids: Vec<ObjectId>,
}
