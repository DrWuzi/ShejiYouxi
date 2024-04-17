use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct AuthRequestResponseSuccess {
    pub login_token: String,
    pub redirect_url: String,
    pub is_console_link_session: bool,
    pub auth_method: String,
    pub puuid: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct AuthRequestResponseMultifactor {
    pub method: String,
    pub methods: Vec<String>,
    pub email: String,
    pub mode: String,
    pub auth_method: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub enum AuthRequestResponseType {
    Success(AuthRequestResponseSuccess),
    Multifactor(AuthRequestResponseMultifactor),
}

#[derive(Debug, Deserialize, Serialize)]
pub struct AuthRequestResponse {
    pub r#type: AuthRequestResponseType,
    pub country: String,
    pub platform: String,
    pub error: Option<String>,
}


