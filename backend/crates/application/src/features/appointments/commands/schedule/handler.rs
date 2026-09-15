use std::sync::{ Arc };
use async_trait::{ async_trait };
use chrono::{ NaiveDateTime };

use domain::aggregates::{
    salons::{ salon::value_objects::SalonId, salon_service::SalonService },
    services::service::{ Service, value_objects::ServiceKind },
    services::service_addon_rule::ServiceAddonRule,
    specialists::{ specialist::Specialist, specialist_service::SpecialistService },
    appointments::appointment::{ Appointment }
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
        <ScheduleAppointmentCommand as Command>::Result,
        <ScheduleAppointmentCommand as Command>::Error
    > {
        let specialist = Specialist::create(command.specialist_id.into(), SalonId::generate(), None, None)?;

        let services: Vec<Service> = command.service_ids
            .iter()
            .enumerate()
            .map(|(index, service_uuid)| {
                Service::restore(
                    (*service_uuid).into(),
                    "Fake service".try_into()?,
                    None,
                    None,
                    "manicure".try_into()?,
                    if index == 0 { ServiceKind::Primary } else { ServiceKind::Addon },
                    60u16.try_into()?,
                    true.into()
                )
            })
            .collect::<Result<Vec<_>, anyhow::Error>>()?;

        if services.is_empty() {
            anyhow::bail!("Cannot schedule an appointment without services");
        }

        let salon_services = services.iter()
            .map(|service| SalonService::create(specialist.salon_id(), service.id(), "0".try_into()?))
            .collect::<Result<Vec<_>, anyhow::Error>>()?;

        let specialist_services = services.iter()
            .map(|service| SpecialistService::create(specialist.id(), specialist.salon_id(), service.id()))
            .collect::<Result<Vec<_>, anyhow::Error>>()?;

        let addon_rules = services.iter().skip(1)
            .map(|service| ServiceAddonRule::create(&services[0], service))
            .collect::<Result<Vec<_>, anyhow::Error>>()?;

        let starts_at = NaiveDateTime::new(command.date, command.time).into();
        
        let appointment = Appointment::create(
            command.client_id.into(),
            &specialist,
            starts_at,
            &services,
            &salon_services,
            &specialist_services,
            &addon_rules
        )?;

        self.service.save_appointment(context, &appointment).await?;

        Ok(())
    }
}
