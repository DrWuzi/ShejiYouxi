use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct ChatMessage {
    pub body: String,
    pub cid: String,
    pub game_name: String,
    pub game_tag: String,
    pub id: String,
    pub mid: String,
    pub name: String,
    pub pid: String,
    pub puuid: String,
    pub read: bool,
    pub region: String,
    /// Time in milliseconds since epoch
    pub time: String,
    pub r#type: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ChatHistoryResponse {
    pub messages: Vec<ChatMessage>,
}
