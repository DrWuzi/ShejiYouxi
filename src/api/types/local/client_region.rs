use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct ClientRegionResponse {
    pub locale: String,
    pub region: String,
    pub web_language: String,
    pub web_region: String,
}
