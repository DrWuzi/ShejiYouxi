use http::Method;

use crate::api::{endpoint::Endpoint, types::local::send_friend_request::{SendFriendReqeustBody, SendFriendRequestResponse}};

#[derive(Debug, Clone)]
pub struct SendFriendRequest {
    pub request_body: SendFriendReqeustBody,
}

impl SendFriendRequest {
    pub fn new(request_body: SendFriendReqeustBody) -> Self {
        Self { request_body }
    }
}

impl Endpoint for SendFriendRequest {
    type Response = SendFriendRequestResponse;
    
    fn method(&self) -> Method {
        Method::POST
    }

    fn endpoint(&self) -> String {
        "chat/v4/friendrequests".to_string()
    }

    fn body(&self) -> Option<Vec<u8>> {
        Some(serde_json::to_vec(&self.request_body).unwrap())
    }
}
