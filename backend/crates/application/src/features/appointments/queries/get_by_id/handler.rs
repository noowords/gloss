use async_trait::{ async_trait };

use crate::common::cqrs::query::{ Query, QueryHandler, QueryContext };
use super::{ GetAppointmentByIdQuery, GetAppointmentByIdQueryService };

pub struct GetAppointmentByIdQueryHandler<S> {
    service: S
}

impl<S> GetAppointmentByIdQueryHandler<S>
where
    S: GetAppointmentByIdQueryService
{
    pub fn build(service: S) -> Self {
        Self { service }
    }
}

#[async_trait]
impl<S> QueryHandler<GetAppointmentByIdQuery> for GetAppointmentByIdQueryHandler<S>
where
    S: GetAppointmentByIdQueryService
{
    async fn handle(&self, context: &dyn QueryContext, query: GetAppointmentByIdQuery) -> Result<
        <GetAppointmentByIdQuery as Query>::View,
        <GetAppointmentByIdQuery as Query>::Error
    > {
        self.service.get_appointment_by_id(context, query.id).await
    }
}
