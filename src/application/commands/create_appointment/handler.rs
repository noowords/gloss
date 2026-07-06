use std::sync::{ Arc };
use async_trait::{ async_trait };

use crate::domain::{
    shared::{ UnitOfWorkFactory, InfrastructureFactory },
    models::{
        user::value_objects::{ UserId },
        appointment::{ Appointment }
    }
};
use super::super::super::shared::{ CommandHandler };

use super::{ CreateAppointmentCommand };

pub struct CreateAppointmentHandler {
    uow_factory: Arc<dyn UnitOfWorkFactory>,
    infra_factory: Arc<dyn InfrastructureFactory>
}

impl CreateAppointmentHandler {
    pub fn new(
        uow_factory: Arc<dyn UnitOfWorkFactory>,
        infra_factory: Arc<dyn InfrastructureFactory>
    ) -> Self {
        Self {
            uow_factory,
            infra_factory
        }
    }
}

#[async_trait]
impl CommandHandler<CreateAppointmentCommand> for CreateAppointmentHandler {
    type Output = ();
    
    async fn handle(&self, cmd: CreateAppointmentCommand) -> Result<Self::Output, Box<dyn std::error::Error + Send + Sync>> {
        let mut uow = self.uow_factory.begin().await?;
        let appointment_repository = self.infra_factory.appointment_repository();

        let appointment = Appointment::new(
            None,
            UserId::from(cmd.master_id),
            UserId::from(cmd.client_id),
            cmd.date,
            cmd.time,
            None
        );

        appointment_repository.create(&mut *uow, &appointment).await?;

        uow.commit().await?;

        Ok(())
    }
}
