use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct ChatSessionResponse {
    pub federated: bool,
    pub game_name: String,
    pub game_tag: String,
    pub loaded: bool,
    pub name: String,
    pub pid: String,
    pub puuid: String,
    pub region: String,
    pub resource: String,
    pub state: String,
}
