use crate::api::{client::{AsyncClient, Client, ClientHost}, error::ApiError};

use super::error::ValorantApiError;

pub struct ValorantApi {
    client: reqwest::blocking::Client,
}

impl ValorantApi {
    pub fn new(client: reqwest::blocking::Client) -> Self {
        Self { client }
    }
}

impl ClientHost for ValorantApi {
    type Error = ValorantApiError;
}

impl Client for ValorantApi {
    fn query(&self, request: http::request::Builder, body: Vec<u8>) -> Result<http::Response<bytes::Bytes>, ApiError<Self::Error>> {
        let call = || -> Result<_, ValorantApiError> {
            let http_request = request.body(body)?;
            let request = http_request.try_into()?;
            let response = self.client.execute(request)?;
            let mut http_response = http::Response::builder()
                .status(response.status())
                .version(response.version());
            let headers = http_response.headers_mut().unwrap();
            for (key, value) in response.headers() {
                headers.insert(key, value.clone());
            }
            Ok(http_response.body(response.bytes()?)?)
        };

        call().map_err(ApiError::client_error)
    }
}

pub struct AsyncValorantApi {
    client: reqwest::Client,
}

impl AsyncValorantApi {
    pub fn new(client: reqwest::Client) -> Self {
        Self { client }
    }
}

impl ClientHost for AsyncValorantApi {
    type Error = ValorantApiError;
}

impl AsyncClient for AsyncValorantApi {
    async fn query_async(&self, request: http::request::Builder, body: Vec<u8>) -> Result<http::Response<bytes::Bytes>, ApiError<Self::Error>> {
        use futures_util::TryFutureExt;
        let call = || {
            async {
                let http_request = request.body(body)?;
                let request = http_request.try_into()?;
                let response = self.client.execute(request).await?;
                let mut http_response = http::Response::builder()
                    .status(response.status())
                    .version(response.version());
                let headers = http_response.headers_mut().unwrap();
                for (key, value) in response.headers() {
                    headers.insert(key, value.clone());
                }
                Ok(http_response.body(response.bytes().await?)?)
            }
        };

        call().map_err(ApiError::client_error).await
    }
}
