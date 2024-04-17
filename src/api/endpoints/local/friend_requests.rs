use http::Method;

use crate::api::{endpoint::Endpoint, types::local::friend_requests::FriendRequestsResponse};

#[derive(Debug, Clone)]
pub struct FriendRequests {
}

impl FriendRequests {
    pub fn new() -> Self {
        Self { }
    }
}

impl Endpoint for FriendRequests {
    type Response = FriendRequestsResponse;
    
    fn method(&self) -> Method {
        Method::GET
    }

    fn endpoint(&self) -> String {
        "chat/v4/friendrequests".to_string()
    }
}
