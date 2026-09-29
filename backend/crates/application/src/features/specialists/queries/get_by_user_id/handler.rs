use async_trait::{ async_trait };

use crate::common::cqrs::query::{ Query, QueryHandler, QueryContext };
use super::{ GetSpecialistByUserIdQuery, GetSpecialistByUserIdQueryService };

pub struct GetSpecialistByUserIdQueryHandler<S> {
    service: S
}

impl<S> GetSpecialistByUserIdQueryHandler<S>
where
    S: GetSpecialistByUserIdQueryService
{
    pub fn build(service: S) -> Self {
        Self { service }
    }
}

#[async_trait]
impl<S> QueryHandler<GetSpecialistByUserIdQuery> for GetSpecialistByUserIdQueryHandler<S>
where
    S: GetSpecialistByUserIdQueryService
{
    async fn handle(&self, context: &dyn QueryContext, query: GetSpecialistByUserIdQuery) -> Result<
        <GetSpecialistByUserIdQuery as Query>::View,
        <GetSpecialistByUserIdQuery as Query>::Error
    > {
        self.service.get_specialist_by_user_id(context, query.user_id).await
    }
}
