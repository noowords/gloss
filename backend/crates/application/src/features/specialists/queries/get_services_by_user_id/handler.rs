use async_trait::{ async_trait };
use std::sync::{ Arc };

use crate::contracts::cqrs::query::{ Query, QueryHandler, QueryContext };
use super::{ GetSpecialistServicesByUserIdQuery, GetSpecialistServicesByUserIdQueryService };

pub struct GetSpecialistServicesByUserIdQueryHandler {
    service: Arc<dyn GetSpecialistServicesByUserIdQueryService>
}

impl GetSpecialistServicesByUserIdQueryHandler {
    pub fn build(service: Arc<dyn GetSpecialistServicesByUserIdQueryService>) -> Self {
        Self { service }
    }
}

#[async_trait]
impl QueryHandler<GetSpecialistServicesByUserIdQuery> for GetSpecialistServicesByUserIdQueryHandler {
    async fn handle(&self, context: &dyn QueryContext, query: GetSpecialistServicesByUserIdQuery) -> Result<
        <GetSpecialistServicesByUserIdQuery as Query>::View,
        <GetSpecialistServicesByUserIdQuery as Query>::Error
    > {
        self.service.get_specialist_services_by_user_id(context, query.user_id).await
    }
}
