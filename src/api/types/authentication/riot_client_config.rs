use std::collections::HashMap;

use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
struct RiotClientConfigResponse {
    #[serde(rename = "chat.affinities")]
    pub chat_affinities: HashMap<String, String>,
    
    #[serde(rename = "chat.affinity_domains")]
    pub chat_affinity_domains: HashMap<String, String>,
    
    #[serde(rename = "chat.port")]
    pub chat_port: u32,

    #[serde(flatten)]
    pub dynamic_properties: HashMap<String, serde_json::Value>,
}
