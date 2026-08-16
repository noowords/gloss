use async_trait::{ async_trait };
use std::sync::{ Arc };

use crate::contracts::cqrs::query::{ Query, QueryHandler, QueryContext };
use super::{ GetAppointmentByIdQuery, GetAppointmentByIdQueryService };

pub struct GetAppointmentByIdQueryHandler {
    service: Arc<dyn GetAppointmentByIdQueryService>
}

impl GetAppointmentByIdQueryHandler {
    pub fn build(service: Arc<dyn GetAppointmentByIdQueryService>) -> Self {
        Self { service }
    }
}

#[async_trait]
impl QueryHandler<GetAppointmentByIdQuery> for GetAppointmentByIdQueryHandler {
    async fn handle(&self, context: &dyn QueryContext, query: GetAppointmentByIdQuery) -> Result<
        <GetAppointmentByIdQuery as Query>::View,
        <GetAppointmentByIdQuery as Query>::Error
    > {
        self.service.get_appointment_by_id(context, query.id).await
    }
}
