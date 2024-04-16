#[derive(Debug, thiserror::Error)]
pub enum ValorantApiError {
    #[error("http error: {0}")]
    Http(#[from] http::Error),
    #[error("communication error: {0}")]
    Communication(#[from] reqwest::Error),
}
