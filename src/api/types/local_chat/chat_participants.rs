use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct ChatParticipant {
    pub active_platform: Option<String>,
    pub cid: String,
    pub game_name: String,
    pub game_tag: String,
    pub muted: bool,
    pub name: String,
    pub pid: String,
    pub puuid: String,
    pub region: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ChatParticipantsResponse {
    pub participants: Vec<ChatParticipant>,
}
