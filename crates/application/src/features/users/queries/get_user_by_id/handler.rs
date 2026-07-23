use async_trait::{ async_trait };
use std::sync::{ Arc };

use crate::interfaces::query::{ Query, QueryHandler, QueryContext };
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
    async fn handle(&self, context: &dyn QueryContext, query: GetUserByIdQuery) -> Result<
        <GetUserByIdQuery as Query>::Result,
        <GetUserByIdQuery as Query>::Error
    > {
        self.service.get_user_by_id(context, query.id)
            .await
            .map(|user| user.into())
    }
}
