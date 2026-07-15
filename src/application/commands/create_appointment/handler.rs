use async_trait::{ async_trait };

use crate::domain::models::{
    user::value_objects::{ UserId },
    appointment::{ Appointment }
};

use super::super::super::common::{
    CommandHandler,
    persistence::{ TxContext, RepositoryFactory }
};

use super::{ CreateAppointmentCommand };

#[derive(Default)]
pub struct CreateAppointmentHandler;

impl CreateAppointmentHandler {
    pub fn new() -> Self {
        Self::default()
    }
}

#[async_trait]
impl CommandHandler<CreateAppointmentCommand> for CreateAppointmentHandler {
    type Output = ();
    type Error = anyhow::Error;
    
    async fn handle(
        &self,
        ctx: &mut dyn TxContext,
        repository_factory: &dyn RepositoryFactory,
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
        
        repository_factory.appointment_repository(ctx)?.create(&appointment).await?;

        Ok(())
    }
}
