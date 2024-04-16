use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct EntitlementsTokenResponse {
    pub access_token: String,
    pub entitlements: Vec<serde_json::Value>,
    pub issuer: String,
    /// Player UUID
    pub subject: String,
    pub token: String,
}
