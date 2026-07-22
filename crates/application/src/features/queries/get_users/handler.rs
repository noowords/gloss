use async_trait::{ async_trait };
use std::sync::{ Arc };

use crate::interfaces::query::{ Query, QueryHandler, QueryContext };
use super::{ GetUsersQuery, GetUsersQueryService };

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
    async fn handle(&self, context: &dyn QueryContext, _query: GetUsersQuery) -> Result<
        <GetUsersQuery as Query>::Result,
        <GetUsersQuery as Query>::Error
    > {
        self.service.get_users(context)
            .await
            .map(|users| users.into())
    }
}
