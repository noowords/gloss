use async_trait::{ async_trait };

use crate::contexts::{ PoolContext };
use super::{ Query };

#[async_trait]
pub trait QueryHandler<Q: Query>: Send + Sync + 'static {
    async fn handle(&self, ctx: &dyn PoolContext, query: Q) -> Result<Q::Result, Q::Error>;
}
