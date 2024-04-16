use std::error::Error;
use bytes::Bytes;
use http::StatusCode;

#[derive(Debug, thiserror::Error)]
pub enum ApiError<E>
where
    E: Error + Send + Sync + 'static,
{
    #[error("client error: {}", source)]
    Client {
        source: E,
    },

    #[error("internal server error {}", status)]
    Server {
        status: StatusCode,
        body: Vec<u8>,
    },

    #[error("moved permanently to: {}", location.as_ref().map(AsRef::as_ref).unwrap_or("<UNKNOWN>"))]
    MovedPermanently {
        location: Option<String>,
    },
    
    #[error("could not parse json: {0}")]
    JsonParse(#[from] serde_json::Error),

    #[error("failed to parse url: {0}")]
    UrlPaseError(#[from] url::ParseError),

    #[error("undefined host url of client and endpoint")]
    UndefinedHostUrl,
}

impl<E> ApiError<E>
where
    E: Error + Send + Sync + 'static,
{
    pub fn client_error(error: E) -> Self {
        Self::Client { source: error }
    }

    pub fn server_error(status: StatusCode, body: &Bytes) -> Self {
        Self::Server { status, body: body.into_iter().copied().collect(), }
    }

    pub fn moved_permanently(location: Option<&http::HeaderValue>) -> Self {
        let location = location.map(|v| String::from_utf8_lossy(v.as_bytes()).into());
        Self::MovedPermanently { location }
    }
}
