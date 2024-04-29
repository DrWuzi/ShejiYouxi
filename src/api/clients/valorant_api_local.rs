use base64::prelude::*;
use http::{header::AUTHORIZATION, HeaderValue};
use url::Url;

use crate::api::{client::{AsyncClient, Client, ClientHost}, error::ApiError, valorant_lockfile::Lockfile};

use super::error::ValorantApiError;

#[derive(Debug, Clone, Default)]
pub struct ValorantApiLocal {
    client: reqwest::blocking::Client,
    lockfile: Lockfile,
}

impl ValorantApiLocal {
    pub fn new(client: reqwest::blocking::Client, lockfile: Lockfile) -> Self {
        Self { client, lockfile }
    }
}

impl ClientHost for ValorantApiLocal {
    type Error = ValorantApiError;

    fn host(&self) -> Result<Option<Url>, ApiError<Self::Error>> {
        Ok(Some(Url::parse(&format!("{}://127.0.0.1:{}/", self.lockfile.protocol, self.lockfile.port))?))
    }
}

impl Client for ValorantApiLocal {
    fn query(&self, mut request: http::request::Builder, body: Vec<u8>) -> Result<http::Response<bytes::Bytes>, ApiError<Self::Error>> {
        let call = || -> Result<_, ValorantApiError> {
            let token = format!("Basic {}", BASE64_STANDARD.encode(format!("riot:{}", self.lockfile)));
            let mut token_value = HeaderValue::from_str(&token).unwrap();
            token_value.set_sensitive(true);
            let headers = request.headers_mut().unwrap();
            headers.insert(AUTHORIZATION, token_value);
          
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

#[derive(Debug, Clone, Default)]
pub struct AsyncValorantApiLocal {
    client: reqwest::Client,
    lockfile: Lockfile
}

impl AsyncValorantApiLocal {
    pub fn new(client: reqwest::Client, lockfile: Lockfile) -> Self {
        Self { client, lockfile }
    }
}

impl ClientHost for AsyncValorantApiLocal {
    type Error = ValorantApiError;

    fn host(&self) -> Result<Option<Url>, ApiError<Self::Error>> {
        Ok(Some(Url::parse(&format!("{}://127.0.0.1:{}/", self.lockfile.protocol, self.lockfile.port))?))
    }
}

impl AsyncClient for AsyncValorantApiLocal {
    async fn query_async(&self, mut request: http::request::Builder, body: Vec<u8>) -> Result<http::Response<bytes::Bytes>, ApiError<Self::Error>> {
        use futures_util::TryFutureExt;
        let call = || {
            async {
                let token = format!("Basic {}", BASE64_STANDARD.encode(format!("riot:{}", self.lockfile.password)));
                let mut token_value = HeaderValue::from_str(&token).unwrap();
                token_value.set_sensitive(true);
                let headers = request.headers_mut().unwrap();
                headers.insert(AUTHORIZATION, token_value);
          
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
