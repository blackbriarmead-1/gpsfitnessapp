use serde::{Deserialize, Serialize};

/// Contains options for activity/me requests
#[derive(Debug, Serialize, Deserialize)]
pub struct ActivityMeOptions {
    pub limit: u32,
    pub offset: u32,
}
