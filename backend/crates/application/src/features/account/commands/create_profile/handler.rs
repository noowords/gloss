use async_trait::{ async_trait };

use domain::aggregates::users::profile::{ Profile };

use crate::common::cqrs::command::{ Command, CommandHandler, CommandContext };
use super::{ CreateAccountProfileCommand, CreateAccountProfileCommandService };

pub struct CreateAccountProfileCommandHandler<S> {
    service: S
}

impl<S> CreateAccountProfileCommandHandler<S>
where
    S: CreateAccountProfileCommandService
{
    pub fn build(service: S) -> Self {
        Self { service }
    }
}

#[async_trait]
impl<S> CommandHandler<CreateAccountProfileCommand> for CreateAccountProfileCommandHandler<S>
where
    S: CreateAccountProfileCommandService
{
    async fn handle(&self, context: &mut dyn CommandContext, command: CreateAccountProfileCommand) -> Result<
        <CreateAccountProfileCommand as Command>::Result,
        <CreateAccountProfileCommand as Command>::Error
    > {
        let profile = Profile::create(
            command.user_id,
            command.first_name,
            command.last_name,
            command.avatar_url
        )?;
        
        self.service.create_profile(context, &profile).await
    }
}
