use async_trait::{ async_trait };
use std::sync::{ Arc };

use crate::interfaces::query::{ Query, QueryHandler, QueryContext };
use super::{ GetUserProfileByIdQuery, GetUserProfileByIdQueryService };

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
    async fn handle(&self, context: &dyn QueryContext, query: GetUserProfileByIdQuery) -> Result<
        <GetUserProfileByIdQuery as Query>::Result,
        <GetUserProfileByIdQuery as Query>::Error
    > {
        self.service.get_user_profile_by_id(context, query.id)
            .await
            .map(|profile| profile.into())
    }
}
