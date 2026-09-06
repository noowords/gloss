use async_trait::{ async_trait };
use std::sync::{ Arc };

use crate::contracts::cqrs::query::{ Query, QueryHandler, QueryContext };
use super::{ GetAppointmentsQuery, GetAppointmentsQueryService };

pub struct GetAppointmentsQueryHandler {
    service: Arc<dyn GetAppointmentsQueryService>
}

impl GetAppointmentsQueryHandler {
    pub fn build(service: Arc<dyn GetAppointmentsQueryService>) -> Self {
        Self { service }
    }
}

#[async_trait]
impl QueryHandler<GetAppointmentsQuery> for GetAppointmentsQueryHandler {
    async fn handle(&self, context: &dyn QueryContext, _query: GetAppointmentsQuery) -> Result<
        <GetAppointmentsQuery as Query>::View,
        <GetAppointmentsQuery as Query>::Error
    > {
        self.service.get_appointments(context).await
    }
}
