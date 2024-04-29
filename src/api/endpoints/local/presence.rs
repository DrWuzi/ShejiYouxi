use http::Method;

use crate::api::{endpoint::Endpoint, types::local::presence::PresenceResponse};

#[derive(Debug, Clone)]
pub struct Presence {
}

impl Presence {
    pub fn new() -> Self {
        Self { }
    }
}

impl Endpoint for Presence {
    type Response = PresenceResponse;
    
    fn method(&self) -> Method {
        Method::GET
    }

    fn endpoint(&self) -> String {
        "chat/v4/presences".to_string()
    }
}
