use async_trait::{ async_trait };

use domain::aggregates::users::profile::{ Profile };

use crate::common::cqrs::command::{ Command, CommandHandler, CommandContext };
use super::{ UpdateAccountProfileCommand, UpdateAccountProfileCommandService };

pub struct UpdateAccountProfileCommandHandler<S> {
    service: S
}

impl<S> UpdateAccountProfileCommandHandler<S>
where
    S: UpdateAccountProfileCommandService
{
    pub fn build(service: S) -> Self {
        Self { service }
    }
}

#[async_trait]
impl<S> CommandHandler<UpdateAccountProfileCommand> for UpdateAccountProfileCommandHandler<S>
where
    S: UpdateAccountProfileCommandService
{
    async fn handle(&self, context: &mut dyn CommandContext, command: UpdateAccountProfileCommand) -> Result<
        <UpdateAccountProfileCommand as Command>::Result,
        <UpdateAccountProfileCommand as Command>::Error
    > {
        let profile = Profile::create(
            command.user_id,
            command.first_name,
            command.last_name,
            command.avatar_url
        )?;
        
        self.service.update_profile(context, &command.user_id, &profile).await
    }
}
