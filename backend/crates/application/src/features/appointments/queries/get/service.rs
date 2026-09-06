use async_trait::{ async_trait };

use crate::contracts::cqrs::query::{ QueryContext };

use super::dtos::{ Appointment };

#[async_trait]
pub trait GetAppointmentsQueryService: Send + Sync {
    async fn get_appointments(&self, context: &dyn QueryContext) -> Result<Vec<Appointment>, anyhow::Error>;
}
