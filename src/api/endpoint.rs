use http::{HeaderMap, Method, Request};
use serde::de::DeserializeOwned;
use url::{ParseError, Url};

use super::{client::{AsyncClient, Client}, error::ApiError, query::{AsyncQuery, Query}};

pub trait Endpoint {
    type Response: DeserializeOwned;
    
    fn method(&self) -> Method;
    fn endpoint(&self) -> String;
    fn host(&self) -> Result<Option<Url>, ParseError> {
        Ok(None)
    }
    fn headers(&self) -> Option<HeaderMap> {
        None
    }
    fn body(&self) -> Option<Vec<u8>> {
        None
    }
}

impl<E, C> Query<E::Response, C> for E
where
    E: Endpoint,
    C: Client,
{
    fn query(&self, client: &C) -> Result<E::Response, ApiError<C::Error>> {
        let endpoint = if let Some(host) = self.host()? {
            host.join(&self.endpoint())?
        } else if let Some(host) = client.host()? {
            host.join(&self.endpoint())?
        } else {
            return Err(ApiError::UndefinedHostUrl);
        };
            
        let mut req = Request::builder()
            .method(self.method())
            .uri(endpoint.as_str());

        if let Some(header_map) = self.headers() {
            for (name, value) in header_map.iter() {
                req = req.header(name, value.clone());
            }
        }
        
        let body = self.body().unwrap_or(Vec::new());
        let response = client.query(req, body)?;
        let status = response.status();
        let value = if let Ok(v) = serde_json::from_slice(response.body()) {
            v
        } else {
            return Err(ApiError::server_error(status, response.body()));
        };
        
        if !status.is_success() {
            return Err(ApiError::server_error(status, response.body()));
        } else if status == http::StatusCode::MOVED_PERMANENTLY {
            return Err(ApiError::moved_permanently(
                response.headers().get(http::header::LOCATION),
            ));
        }

        serde_json::from_value::<E::Response>(value).map_err(ApiError::JsonParse)
    }
}

impl<E, C> AsyncQuery<E::Response, C> for E
where
    E: Endpoint,
    C: AsyncClient,
{
    async fn query_async(&self, client: &C) -> Result<E::Response, ApiError<C::Error>> {
        let endpoint = if let Some(host) = self.host()? {
            host.join(&self.endpoint())?
        } else if let Some(host) = client.host()? {
            host.join(&self.endpoint())?
        } else {
            return Err(ApiError::UndefinedHostUrl);
        };
        
        let mut req = Request::builder()
            .method(self.method())
            .uri(endpoint.as_str());

        if let Some(header_map) = self.headers() {
            for (name, value) in header_map.iter() {
                req = req.header(name, value.clone());
            }
        }
        
        let body = self.body().unwrap_or(Vec::new());
        let response = client.query_async(req, body).await?;
        let status = response.status();
        let value = if let Ok(v) = serde_json::from_slice(response.body()) {
            v
        } else {
            return Err(ApiError::server_error(status, response.body()));
        };
        
        if !status.is_success() {
            return Err(ApiError::server_error(status, response.body()));
        } else if status == http::StatusCode::MOVED_PERMANENTLY {
            return Err(ApiError::moved_permanently(
                response.headers().get(http::header::LOCATION),
            ));
        }

        serde_json::from_value::<E::Response>(value).map_err(ApiError::JsonParse)
    }
}
