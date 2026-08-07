use std::sync::{ Arc };
use async_trait::{ async_trait };

use domain::aggregates::{
    user::{ User },
    profile::{ Profile }
};

use crate::contracts::cqrs::command::{ Command, CommandHandler, CommandContext };
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
        (), <RegisterUserCommand as Command>::Error
    > {
        let user = User::create();

        self.service.save_user(context, &user).await?;

        let profile = Profile::create(
            user.id(),
            command.first_name.into(),
            command.last_name
                .map(|ln| ln.as_str().try_into())
                .transpose()?,
            command.avatar_url
                .map(|au| au.as_str().try_into())
                .transpose()?,
            None
        );
        
        self.service.save_profile(context, &profile).await?;

        Ok(())
    }
}
