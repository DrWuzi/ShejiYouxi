use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct FriendRequest {
    pub game_name: String,
    pub game_tag: String,
    pub name: String,
    pub note: String,
    pub pid: String,
    pub uuid: String,
    pub region: String,
    pub subscription: FriendRequestSubscription,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum FriendRequestSubscription {
    PendingOut,
    PendingIn,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct FriendRequestsResponse {
    pub requests: Vec<FriendRequest>,
}
