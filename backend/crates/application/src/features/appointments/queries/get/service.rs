use async_trait::{ async_trait };

use crate::common::cqrs::query::{ QueryContext };

use super::dtos::{ Appointment };

#[async_trait]
pub trait GetAppointmentsQueryService: Send + Sync + 'static {
    async fn get_appointments(&self, context: &dyn QueryContext) -> Result<Vec<Appointment>, anyhow::Error>;
}
