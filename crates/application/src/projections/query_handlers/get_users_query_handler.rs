use async_trait::{ async_trait };
use std::sync::{ Arc };

use crate::contexts::{ PoolContext };
use crate::buses::query_bus::{ Query, QueryHandler };

use crate::projections::{
    queries::{ GetUsersQuery },
    query_services::{ GetUsersQueryService }
};

pub struct GetUsersQueryHandler {
    service: Arc<dyn GetUsersQueryService>
}

impl GetUsersQueryHandler {
    pub fn build(service: Arc<dyn GetUsersQueryService>) -> Self {
        Self { service }
    }
}

#[async_trait]
impl QueryHandler<GetUsersQuery> for GetUsersQueryHandler {
    async fn handle(&self, ctx: &dyn PoolContext, _query: GetUsersQuery) -> Result<
        <GetUsersQuery as Query>::Result,
        <GetUsersQuery as Query>::Error
    > {
        self.service.get_users(ctx)
            .await
            .map(|users| users.into())
    }
}
