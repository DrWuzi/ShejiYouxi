use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct PresenceResponse {
    pub presences: Vec<PresenceDetails>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct PresenceDetails {
    pub actor: Option<Value>,
    pub basic: String,
    pub details: Option<Value>,
    pub game_name: String,
    pub game_tag: String,
    pub location: Option<Value>,
    pub msg: Option<Value>,
    pub name: String,
    pub patchline: Option<Value>,
    pub pid: String,
    pub platform: Option<Value>,
    pub private: Option<String>,
    pub private_jwt: Option<Value>,
    pub product: ProductType,
    pub puuid: String,
    pub region: String,
    pub resource: String,
    pub state: PresenceState,
    pub summary: String,
    pub time: u64,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum PresenceState {
    Mobile,
    Dnd,
    Away,
    Chat,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum ProductType {
    Valorant,
    LeagueOfLegends,
}
