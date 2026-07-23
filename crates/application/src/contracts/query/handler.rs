use async_trait::{ async_trait };

use super::{ Query, QueryContext };

#[async_trait]
pub trait QueryHandler<Q: Query>: Send + Sync + 'static {
    async fn handle(&self, context: &dyn QueryContext, query: Q) -> Result<Q::Result, Q::Error>;
}
