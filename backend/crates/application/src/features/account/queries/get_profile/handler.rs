use async_trait::{ async_trait };
use std::sync::{ Arc };

use crate::contracts::cqrs::query::{ Query, QueryHandler, QueryContext };

use super::{ GetAccountProfileQuery, GetAccountProfileQueryService };

pub struct GetAccountProfileQueryHandler {
    service: Arc<dyn GetAccountProfileQueryService>
}

impl GetAccountProfileQueryHandler {
    pub fn build(service: Arc<dyn GetAccountProfileQueryService>) -> Self {
        Self { service }
    }
}

#[async_trait]
impl QueryHandler<GetAccountProfileQuery> for GetAccountProfileQueryHandler {
    async fn handle(&self, context: &dyn QueryContext, query: GetAccountProfileQuery) -> Result<
        <GetAccountProfileQuery as Query>::View,
        <GetAccountProfileQuery as Query>::Error
    > {
        self.service.get_profile_by_user_id(context, query.user_id).await
    }
}
