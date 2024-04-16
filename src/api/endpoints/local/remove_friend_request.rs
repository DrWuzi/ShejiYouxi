use http::Method;

use crate::api::endpoint::Endpoint;

#[derive(Debug, Clone)]
pub struct RemoveFriendRequest;

impl RemoveFriendRequest {
    pub fn new() -> Self {
        Self { }
    }
}

impl Endpoint for RemoveFriendRequest {
    type Response = (); // TODO: Create IgnoreEndpoint for endpoints without return value
    
    fn method(&self) -> Method {
        Method::DELETE
    }

    fn endpoint(&self) -> String {
        "chat/v4/friendrequests".to_string()
    }
}
