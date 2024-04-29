use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Friend {
    #[serde(rename = "activePlatform")]
    pub active_platform: Option<String>,
    #[serde(rename = "displayGroup")]
    pub display_group: String,
    pub game_name: String,
    pub game_tag: String,
    pub group: String,
    pub last_online_ts: Option<u64>,
    pub name: String,
    pub note: String,
    pub pid: String,
    pub puuid: String,
    pub region: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct FriendsResponse {
    pub friends: Vec<Friend>,
}
