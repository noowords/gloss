use std::sync::{ Arc };
use async_trait::{ async_trait };

use domain::appointment::{ Appointment };

use crate::contexts::{ TxContext };
use crate::buses::command_bus::{ Command, CommandHandler };
use super::{ ScheduleAppointmentCommand, ScheduleAppointmentCommandService };

pub struct ScheduleAppointmentCommandHandler {
    service: Arc<dyn ScheduleAppointmentCommandService>
}

impl ScheduleAppointmentCommandHandler {
    pub fn build(service: Arc<dyn ScheduleAppointmentCommandService>) -> Self {
        Self { service }
    }
}

#[async_trait]
impl CommandHandler<ScheduleAppointmentCommand> for ScheduleAppointmentCommandHandler {
    async fn handle(&self, ctx: &mut dyn TxContext, command: ScheduleAppointmentCommand) -> Result<
        <ScheduleAppointmentCommand as Command>::Result,
        <ScheduleAppointmentCommand as Command>::Error
    > {
        let appointment = Appointment::schedule(
            command.master_id.into(),
            command.client_id.into(),
            command.date,
            command.time
        );

        self.service.save_appointment(ctx, &appointment).await?;
        
        Ok(())
    }
}
