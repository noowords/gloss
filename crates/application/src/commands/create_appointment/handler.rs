use async_trait::{ async_trait };

use domain::models::{
    user::value_objects::{ UserId },
    appointment::{ Appointment }
};

use super::super::super::common::{
    commands::{ CommandHandler },
    persistence::{ TxContext, RepositoryFactory }
};

use super::{ CreateAppointmentCommand };

#[derive(Default)]
pub struct CreateAppointmentHandler;

#[async_trait]
impl CommandHandler<CreateAppointmentCommand> for CreateAppointmentHandler {
    async fn handle(
        &self,
        ctx: &mut dyn TxContext,
        repository_factory: &dyn RepositoryFactory,
        command: CreateAppointmentCommand
    ) -> Result<(), anyhow::Error> {
        let appointment = Appointment::new(
            None,
            UserId::from(command.master_id),
            UserId::from(command.client_id),
            command.date,
            command.time,
            None
        );
        
        repository_factory.appointments(ctx)?.create(&appointment).await?;

        Ok(())
    }
}
