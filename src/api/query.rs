use super::{client::{AsyncClient, Client}, error::ApiError};

pub trait Query<T, C> 
where
    C: Client
{
    fn query(&self, client: &C) -> Result<T, ApiError<C::Error>>;
}

pub trait AsyncQuery<T, C> 
where
    C: AsyncClient
{
    async fn query_async(&self, client: &C) -> Result<T, ApiError<C::Error>>;
}
