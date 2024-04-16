use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
pub enum ProvisioningFlow {
    Matchmaking,
    CustomGame,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
pub enum TeamID {
    Blue,
    Red,
    Other(String),
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct GamePlayerResponse {
    /// Player UUID
    pub subject: String,
    #[serde(rename = "MatchID")]
    pub match_id: String,
    pub version: u32,
}
