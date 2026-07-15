use std::sync::{ Arc };
use async_trait::{ async_trait };

use crate::domain::models::{
    user::value_objects::{ UserId },
    appointment::{ Appointment, AppointmentRepository }
};

use super::super::super::common::{
    CommandHandler,
    persistence::{ TxContext }
};

use super::{ CreateAppointmentCommand };

pub struct CreateAppointmentHandler {
    appointment_repository: Arc<dyn AppointmentRepository>
}

impl CreateAppointmentHandler {
    pub fn new(
        appointment_repository: Arc<dyn AppointmentRepository>
    ) -> Self {
        Self { appointment_repository }
    }
}

#[async_trait]
impl CommandHandler<CreateAppointmentCommand> for CreateAppointmentHandler {
    type Output = ();
    type Error = anyhow::Error;
    
    async fn handle(
        &self,
        ctx: &mut dyn TxContext,
        command: CreateAppointmentCommand
    ) -> Result<Self::Output, Self::Error> {
        let appointment = Appointment::new(
            None,
            UserId::from(command.master_id),
            UserId::from(command.client_id),
            command.date,
            command.time,
            None
        );

        self.appointment_repository.create(ctx, &appointment).await?;

        Ok(())
    }
}
