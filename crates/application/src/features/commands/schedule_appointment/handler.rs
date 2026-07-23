use std::sync::{ Arc };
use async_trait::{ async_trait };

use domain::aggregates::appointment::{ Appointment };

use crate::interfaces::command::{ Command, CommandHandler, CommandContext };
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
    async fn handle(&self, context: &mut dyn CommandContext, command: ScheduleAppointmentCommand) -> Result<
        <ScheduleAppointmentCommand as Command>::Result,
        <ScheduleAppointmentCommand as Command>::Error
    > {
        let appointment = Appointment::schedule(
            command.master_id.into(),
            command.client_id.into(),
            command.date,
            command.time
        );

        self.service.save_appointment(context, &appointment).await?;

        Ok(())
    }
}
