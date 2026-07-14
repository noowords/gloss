use std::sync::{ Arc };
use async_trait::{ async_trait };

use crate::domain::{
    shared::{ UnitOfWorkFactory },
    models::{
        user::value_objects::{ UserId },
        appointment::{ Appointment, AppointmentRepository }
    }
};
use super::super::super::shared::{ CommandHandler };

use super::{ CreateAppointmentCommand };

pub struct CreateAppointmentHandler {
    uow_factory: Arc<dyn UnitOfWorkFactory>,
    appointment_repository: Arc<dyn AppointmentRepository>
}

impl CreateAppointmentHandler {
    pub fn new(
        uow_factory: Arc<dyn UnitOfWorkFactory>,
        appointment_repository: Arc<dyn AppointmentRepository>
    ) -> Self {
        Self { uow_factory, appointment_repository }
    }
}

#[async_trait]
impl CommandHandler<CreateAppointmentCommand> for CreateAppointmentHandler {
    type Output = ();
    
    async fn handle(&self, command: CreateAppointmentCommand) -> Result<Self::Output, Box<dyn std::error::Error + Send + Sync>> {
        let mut uow = self.uow_factory.begin().await?;

        let appointment = Appointment::new(
            None,
            UserId::from(command.master_id),
            UserId::from(command.client_id),
            command.date,
            command.time,
            None
        );

        self.appointment_repository.create(uow.ctx_mut(), &appointment).await?;

        uow.commit().await?;

        Ok(())
    }
}
