use async_trait::{ async_trait };

use domain::appointment::{ Appointment };

use crate::contexts::{ TxContext };

#[async_trait]
pub trait ScheduleAppointmentCommandService: Send + Sync {
    async fn save_appointment(&self, ctx: &mut dyn TxContext, appointment: &Appointment) -> Result<(), anyhow::Error>;
}
