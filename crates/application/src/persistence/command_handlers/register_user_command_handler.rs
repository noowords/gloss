use std::sync::{ Arc };
use async_trait::{ async_trait };

use domain::user::{ User };

use crate::contexts::{ TxContext };
use crate::buses::command_bus::{ Command, CommandHandler };
use crate::persistence::{
    commands::{ RegisterUserCommand },
    command_services::{ RegisterUserCommandService }
};

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
    async fn handle(&self, ctx: &mut dyn TxContext, command: RegisterUserCommand) -> Result<
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

        self.service.save_user(ctx, &user).await?;
        self.service.save_profile(ctx, &user.profile()).await?;

        Ok(())
    }
}
