use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct PreGameLoadoutsResponse {
    pub loadouts: Vec<Loadout>,
    pub loadouts_valid: bool,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct Loadout {
    pub subject: String,
    pub sprays: Option<Sprays>,
    pub expressions: Option<Expressions>,
    pub items: HashMap<String, Item>,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct Sprays {
    pub spray_selections: Vec<SpraySelection>,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct SpraySelection {
    #[serde(rename = "SocketID")]
    pub socket_id: String,
    #[serde(rename = "SprayID")]
    pub spray_id: String,
    #[serde(rename = "LevelID")]
    pub level_id: String,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct Expressions {
    #[serde(rename = "AESSelection")]
    pub aes_selections: Vec<AESSelection>,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct AESSelection {
    #[serde(rename = "SocketID")]
    pub socket_id: String,
    #[serde(rename = "AssetID")]
    pub asset_id: String,
    #[serde(rename = "TypeID")]
    pub type_id: String,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct Item {
    #[serde(rename = "ID")]
    pub id: String,
    #[serde(rename = "TypeID")]
    pub type_id: String,
    pub sockets: HashMap<String, Socket>,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct Socket {
    #[serde(rename = "ID")]
    pub id: String,
    pub item: InnerItem,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct InnerItem {
    #[serde(rename = "ID")]
    pub id: String,
    #[serde(rename = "TypeID")]
    pub type_id: String,
}
