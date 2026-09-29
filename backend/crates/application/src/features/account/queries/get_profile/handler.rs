use async_trait::{ async_trait };

use crate::common::cqrs::query::{ Query, QueryHandler, QueryContext };

use super::{ GetAccountProfileQuery, GetAccountProfileQueryService };

pub struct GetAccountProfileQueryHandler<S> {
    service: S
}

impl<S> GetAccountProfileQueryHandler<S>
where
    S: GetAccountProfileQueryService
{
    pub fn build(service: S) -> Self {
        Self { service }
    }
}

#[async_trait]
impl<S> QueryHandler<GetAccountProfileQuery> for GetAccountProfileQueryHandler<S>
where
    S: GetAccountProfileQueryService
{
    async fn handle(&self, context: &dyn QueryContext, query: GetAccountProfileQuery) -> Result<
        <GetAccountProfileQuery as Query>::View,
        <GetAccountProfileQuery as Query>::Error
    > {
        self.service.get_profile_by_user_id(context, query.user_id).await
    }
}
