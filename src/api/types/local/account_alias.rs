use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct AccountAliasResponse {
    pub active: bool,
    pub created_timestamp: u64,
    pub game_name: String,
    pub summoner: bool,
    pub tag_line: String,
}
