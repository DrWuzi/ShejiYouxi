use std::collections::HashMap;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct LaunchConfiguration {
    pub arguments: Vec<String>,
    pub executable: String,
    pub locale: Option<String>,
    pub voice_locale: Option<String>,
    pub working_directory: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Session {
    pub exit_code: u32,
    pub exit_reason: Option<serde_json::Value>,
    pub is_internal: bool,
    pub launch_configuration: LaunchConfiguration,
    pub patchline_full_name: PatchlineFullName,
    pub patchline_id: PatchlineId,
    pub phase: String,
    pub product_id: ProductId,
    pub version: String,
}

#[derive(Debug, Deserialize, Serialize)]
    #[serde(rename_all = "snake_case")]
pub enum PatchlineFullName {
    #[serde(rename = "VALORANT")]
    Valorant,
    RiotClient,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum PatchlineId {
    #[serde(rename = "")]
    Empty,
    Live,
    PBE
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum ProductId {
    Valorant,
    RiotClient,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct SessionsResponse {
    #[serde(flatten)]
    pub sessions: HashMap<String, Session>,
}
