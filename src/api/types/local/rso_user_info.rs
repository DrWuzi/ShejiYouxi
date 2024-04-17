use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct RSOUserInfoResponse {
    pub user_info: String,
}
