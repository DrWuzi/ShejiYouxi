use std::collections::HashMap;

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct PlayerInfoResponse {
    pub country: String,
    /// Player UUID
    pub sub: String,
    pub email_verified: bool,
    pub player_plocale: Option<serde_json::Value>,
    /// Milliseconds since epoch
    pub country_at: Option<i64>,
    pub pw: PlayerPasswordInfo,
    pub phone_number_verified: bool,
    pub account_verified: bool,
    pub ppid: Option<serde_json::Value>,
    pub federated_identity_providers: Vec<String>,
    pub player_locale: Option<String>,
    pub acct: AccountInfo,
    pub age: i32,
    pub jti: String,
    pub affinity: HashMap<String, String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PlayerPasswordInfo {
    /// Milliseconds since epoch
    pub cng_at: i64,
    pub reset: bool,
    pub must_reset: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AccountInfo {
    pub r#type: i32,
    pub state: String,
    pub adm: bool,
    pub game_name: String,
    pub tag_line: String,
    /// Milliseconds since epoch
    pub created_at: i64,
}
