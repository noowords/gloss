use async_trait::{ async_trait };
use std::sync::{ Arc };

use crate::contracts::cqrs::query::{ Query, QueryHandler, QueryContext };
use super::{ GetSpecialistByUserIdQuery, GetSpecialistByUserIdQueryService };

pub struct GetSpecialistByUserIdQueryHandler {
    service: Arc<dyn GetSpecialistByUserIdQueryService>
}

impl GetSpecialistByUserIdQueryHandler {
    pub fn build(service: Arc<dyn GetSpecialistByUserIdQueryService>) -> Self {
        Self { service }
    }
}

#[async_trait]
impl QueryHandler<GetSpecialistByUserIdQuery> for GetSpecialistByUserIdQueryHandler {
    async fn handle(&self, context: &dyn QueryContext, query: GetSpecialistByUserIdQuery) -> Result<
        <GetSpecialistByUserIdQuery as Query>::View,
        <GetSpecialistByUserIdQuery as Query>::Error
    > {
        self.service.get_specialist_by_user_id(context, query.user_id).await
    }
}
