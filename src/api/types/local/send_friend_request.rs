use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "snake_case")]
pub struct SendFriendRequestResponse {
    #[serde(default)]
    pub requests: Vec<serde_json::Value>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
#[serde(rename_all = "snake_case")]
pub struct SendFriendReqeustBody {
    pub game_name: String,
    pub game_tag: String,
}
