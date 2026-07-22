use async_trait::{ async_trait };
use std::sync::{ Arc };

use crate::contexts::{ PoolContext };
use crate::{ Query, QueryHandler };
use super::{ GetUserByIdQuery, GetUserByIdQueryService };

pub struct GetUserByIdQueryHandler {
    service: Arc<dyn GetUserByIdQueryService>
}

impl GetUserByIdQueryHandler {
    pub fn build(service: Arc<dyn GetUserByIdQueryService>) -> Self {
        Self { service }
    }
}

#[async_trait]
impl QueryHandler<GetUserByIdQuery> for GetUserByIdQueryHandler {
    async fn handle(&self, ctx: &dyn PoolContext, query: GetUserByIdQuery) -> Result<
        <GetUserByIdQuery as Query>::Result,
        <GetUserByIdQuery as Query>::Error
    > {
        self.service.get_user_by_id(ctx, query.id)
            .await
            .map(|user| user.into())
    }
}
