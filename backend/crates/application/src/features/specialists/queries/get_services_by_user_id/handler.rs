use async_trait::{ async_trait };

use crate::common::cqrs::query::{ Query, QueryHandler, QueryContext };
use super::{ GetSpecialistServicesByUserIdQuery, GetSpecialistServicesByUserIdQueryService };

pub struct GetSpecialistServicesByUserIdQueryHandler<S> {
    service: S
}

impl<S> GetSpecialistServicesByUserIdQueryHandler<S>
where
    S: GetSpecialistServicesByUserIdQueryService
{
    pub fn build(service: S) -> Self {
        Self { service }
    }
}

#[async_trait]
impl<S> QueryHandler<GetSpecialistServicesByUserIdQuery> for GetSpecialistServicesByUserIdQueryHandler<S>
where
    S: GetSpecialistServicesByUserIdQueryService
{
    async fn handle(&self, context: &dyn QueryContext, query: GetSpecialistServicesByUserIdQuery) -> Result<
        <GetSpecialistServicesByUserIdQuery as Query>::View,
        <GetSpecialistServicesByUserIdQuery as Query>::Error
    > {
        self.service.get_specialist_services_by_user_id(context, query.user_id).await
    }
}
