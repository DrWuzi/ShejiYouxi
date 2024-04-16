use std::error::Error;

use bytes::Bytes;
use http::{request::Builder, Response};
use url::Url;

use super::error::ApiError;

pub trait ClientHost {
    type Error: Error + Send + Sync + 'static;

    fn host(&self) -> Result<Option<Url>, ApiError<Self::Error>> {
        Ok(None)
    }
}

pub trait Client: ClientHost {
    fn query(&self, request: Builder, body: Vec<u8>) -> Result<Response<Bytes>, ApiError<Self::Error>>;
}

pub trait AsyncClient: ClientHost {
    async fn query_async(&self, request: Builder, body: Vec<u8>) -> Result<Response<Bytes>, ApiError<Self::Error>>;
}
