use std::sync::{ Arc };
use async_trait::{ async_trait };

use domain::aggregates::user::{ User };

use crate::contracts::command::{ Command, CommandHandler, CommandContext };
use super::{ RegisterUserCommand, RegisterUserCommandService };

pub struct RegisterUserCommandHandler {
    service: Arc<dyn RegisterUserCommandService>
}

impl RegisterUserCommandHandler {
    pub fn build(service: Arc<dyn RegisterUserCommandService>) -> Self {
        Self { service }
    }
}

#[async_trait]
impl CommandHandler<RegisterUserCommand> for RegisterUserCommandHandler {
    async fn handle(&self, context: &mut dyn CommandContext, command: RegisterUserCommand) -> Result<
        <RegisterUserCommand as Command>::Result,
        <RegisterUserCommand as Command>::Error
    > {
        let user = User::register(
            command.phone
                .map(|p| p.as_str().try_into())
                .transpose()?,
            command.first_name,
            command.last_name,
            command.avatar_url
        );

        self.service.save_user(context, &user).await?;
        self.service.save_profile(context, &user.profile()).await?;

        Ok(())
    }
}
