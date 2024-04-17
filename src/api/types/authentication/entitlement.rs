use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct EntitlementResponse {
    pub entitlements_token: String,
}
