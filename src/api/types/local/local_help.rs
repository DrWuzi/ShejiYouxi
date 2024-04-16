use std::collections::HashMap;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct LocalHelpResponse {
    pub events: HashMap<String, String>,
    pub functions: HashMap<String, String>,
    pub types: HashMap<String, String>,
}
