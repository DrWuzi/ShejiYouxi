use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct RiotGeoResponse {
    pub token: String,
    pub affinities: Affinities,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Affinities {
    pub pbe: String,
    pub live: String,
}
