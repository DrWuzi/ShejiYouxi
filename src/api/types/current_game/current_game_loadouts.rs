use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct CurrentGameLoadoutsResponse {
    pub loadouts: Vec<Loadout>,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct Loadout {
    #[serde(rename = "CharacterID")]
    pub character_id: String,
    pub loadout: LoadoutDetails,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct LoadoutDetails {
    pub subject: String,
    pub sprays: Option<Sprays>,
    pub expressions: Option<Expressions>,
    pub items: HashMap<String, LoadoutItem>,
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
    #[serde(rename = "AESSelections")]
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
pub struct LoadoutItem {
    #[serde(rename = "ID")]
    pub id: String,
    #[serde(rename = "TypeID")]
    pub type_id: String,
    pub sockets: HashMap<String, LoadoutSocket>,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct LoadoutSocket {
    #[serde(rename = "ID")]
    pub id: String,
    pub item: LoadoutItemDetails,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct LoadoutItemDetails {
    #[serde(rename = "ID")]
    pub id: String,
    #[serde(rename = "TypeID")]
    pub type_id: String,
}
