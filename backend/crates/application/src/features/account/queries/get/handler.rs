use async_trait::{ async_trait };
use std::sync::{ Arc };

use crate::contracts::cqrs::query::{ Query, QueryHandler, QueryContext };
use super::{ GetAccountQuery, GetAccountQueryService };

pub struct GetAccountQueryHandler {
    service: Arc<dyn GetAccountQueryService>
}

impl GetAccountQueryHandler {
    pub fn build(service: Arc<dyn GetAccountQueryService>) -> Self {
        Self { service }
    }
}

#[async_trait]
impl QueryHandler<GetAccountQuery> for GetAccountQueryHandler {
    async fn handle(&self, context: &dyn QueryContext, query: GetAccountQuery) -> Result<
        <GetAccountQuery as Query>::View,
        <GetAccountQuery as Query>::Error
    > {
        self.service.get_user_by_id(context, query.id).await
    }
}
