use async_trait::{ async_trait };

use crate::common::cqrs::query::{ Query, QueryHandler, QueryContext };
use super::{ GetAppointmentsQuery, GetAppointmentsQueryService };

pub struct GetAppointmentsQueryHandler<S> {
    service: S
}

impl<S> GetAppointmentsQueryHandler<S>
where
    S: GetAppointmentsQueryService
{
    pub fn build(service: S) -> Self {
        Self { service }
    }
}

#[async_trait]
impl<S> QueryHandler<GetAppointmentsQuery> for GetAppointmentsQueryHandler<S>
where
    S: GetAppointmentsQueryService
{
    async fn handle(&self, context: &dyn QueryContext, _query: GetAppointmentsQuery) -> Result<
        <GetAppointmentsQuery as Query>::View,
        <GetAppointmentsQuery as Query>::Error
    > {
        self.service.get_appointments(context).await
    }
}
