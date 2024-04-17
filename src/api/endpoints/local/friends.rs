use http::Method;

use crate::api::{endpoint::Endpoint, types::local::friends::FriendsResponse};

#[derive(Debug, Clone)]
pub struct Friends {
}

impl Friends {
    pub fn new() -> Self {
        Self { }
    }
}

impl Endpoint for Friends {
    type Response = FriendsResponse;
    
    fn method(&self) -> Method {
        Method::GET
    }

    fn endpoint(&self) -> String {
        "chat/v4/friends".to_string()
    }
}
