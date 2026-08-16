use std::sync::{ Arc };
use async_trait::{ async_trait };

use domain::aggregates::{
    service::{ Service },
    appointment::{ Appointment }
};

use crate::contracts::cqrs::command::{ Command, CommandHandler, CommandContext };
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
        (), <ScheduleAppointmentCommand as Command>::Error
    > {
        let services: Vec<Service> = command.service_ids
            .iter()
            .map(|service_uuid| {
                Service::restore(
                    (*service_uuid).into(),
                    "manicure".try_into().unwrap(),
                    "Fake service".into(),
                    None,
                    None,
                    "1500.00".try_into().unwrap(),
                    60.into(),
                    true.into()
                )
            })
            .collect();
        
        let appointment = Appointment::create(
            command.specialist_id.into(),
            command.client_id.into(),
            command.date.into(),
            command.time.into(),
            &services
        ).map_err(|e| anyhow::anyhow!(e))?;

        self.service.save_appointment(context, &appointment).await?;

        Ok(())
    }
}
