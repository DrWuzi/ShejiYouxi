use crate::api::{self, clients::error::ValorantApiError};

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("generic error: {0}")]
    Generic(String),

    #[error("api error: {0}")]
    ApiError(#[from] api::ApiError<ValorantApiError>),

    #[error("app error: {0}")]
    AppError(#[from] iced::Error),
}

pub type Result<T> = std::result::Result<T, Error>;
