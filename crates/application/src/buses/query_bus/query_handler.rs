use async_trait::{ async_trait };

use super::{ Query };

#[async_trait]
pub trait QueryHandler<Q: Query>: Send + Sync {
    type Output: Send + 'static;
    async fn handle(&self, query: Q) -> Result<Self::Output, Box<dyn std::error::Error + Send + Sync>>;
}
