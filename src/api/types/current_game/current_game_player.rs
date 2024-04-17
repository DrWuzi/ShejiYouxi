use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct CurrentGamePlayerResponse {
    /// Player UUID
    pub subject: String,
    #[serde(rename = "MatchID")]
    pub match_id: String,
    pub version: u32,
}
