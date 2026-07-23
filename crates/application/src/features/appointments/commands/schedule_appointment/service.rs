use async_trait::{ async_trait };

use domain::aggregates::appointment::{ Appointment };

use crate::contracts::command::{ CommandContext };

#[async_trait]
pub trait ScheduleAppointmentCommandService: Send + Sync {
    async fn save_appointment(&self, ctx: &mut dyn CommandContext, appointment: &Appointment) -> Result<(), anyhow::Error>;
}
