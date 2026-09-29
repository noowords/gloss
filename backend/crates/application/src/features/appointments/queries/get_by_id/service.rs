use async_trait::{ async_trait };

use domain::aggregates::appointments::appointment::value_objects::{ AppointmentId };

use crate::common::cqrs::query::{ QueryContext };

use super::dtos::{ Appointment };

#[async_trait]
pub trait GetAppointmentByIdQueryService: Send + Sync + 'static {
    async fn get_appointment_by_id(&self, context: &dyn QueryContext, id: AppointmentId) -> Result<Option<Appointment>, anyhow::Error>;
}
