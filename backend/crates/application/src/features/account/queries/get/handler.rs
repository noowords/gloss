use async_trait::{ async_trait };

use crate::common::cqrs::query::{ Query, QueryHandler, QueryContext };
use super::{ GetAccountQuery, GetAccountQueryService };

pub struct GetAccountQueryHandler<S> {
    service: S
}

impl<S> GetAccountQueryHandler<S>
where
    S: GetAccountQueryService
{
    pub fn build(service: S) -> Self {
        Self { service }
    }
}

#[async_trait]
impl<S> QueryHandler<GetAccountQuery> for GetAccountQueryHandler<S>
where
    S: GetAccountQueryService
{
    async fn handle(&self, context: &dyn QueryContext, query: GetAccountQuery) -> Result<
        <GetAccountQuery as Query>::View,
        <GetAccountQuery as Query>::Error
    > {
        self.service.get_user_by_id(context, query.id).await
    }
}
