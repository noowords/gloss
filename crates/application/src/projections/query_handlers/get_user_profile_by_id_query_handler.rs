use async_trait::{ async_trait };
use std::sync::{ Arc };

use crate::contexts::{ PoolContext };
use crate::buses::query_bus::{ Query, QueryHandler };

use crate::projections::{
    queries::{ GetUserProfileByIdQuery },
    query_services::{ GetUserProfileByIdQueryService }
};

pub struct GetUserProfileByIdQueryHandler {
    service: Arc<dyn GetUserProfileByIdQueryService>
}

impl GetUserProfileByIdQueryHandler {
    pub fn build(service: Arc<dyn GetUserProfileByIdQueryService>) -> Self {
        Self { service }
    }
}

#[async_trait]
impl QueryHandler<GetUserProfileByIdQuery> for GetUserProfileByIdQueryHandler {
    async fn handle(&self, ctx: &dyn PoolContext, query: GetUserProfileByIdQuery) -> Result<
        <GetUserProfileByIdQuery as Query>::Result,
        <GetUserProfileByIdQuery as Query>::Error
    > {
        self.service.get_user_profile_by_id(ctx, query.id)
            .await
            .map(|profile| profile.into())
    }
}
