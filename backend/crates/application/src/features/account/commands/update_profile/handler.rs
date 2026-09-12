use std::sync::{ Arc };
use async_trait::{ async_trait };

use domain::aggregates::profile::{ Profile };

use crate::contracts::cqrs::command::{ Command, CommandHandler, CommandContext };
use super::{ UpdateAccountProfileCommand, UpdateAccountProfileCommandService };

pub struct UpdateAccountProfileCommandHandler {
    service: Arc<dyn UpdateAccountProfileCommandService>
}

impl UpdateAccountProfileCommandHandler {
    pub fn build(service: Arc<dyn UpdateAccountProfileCommandService>) -> Self {
        Self { service }
    }
}

#[async_trait]
impl CommandHandler<UpdateAccountProfileCommand> for UpdateAccountProfileCommandHandler {
    async fn handle(&self, context: &mut dyn CommandContext, command: UpdateAccountProfileCommand) -> Result<
        <UpdateAccountProfileCommand as Command>::Result,
        <UpdateAccountProfileCommand as Command>::Error
    > {
        let profile = Profile::create(
            command.user_id,
            command.first_name,
            command.last_name,
            command.avatar_url,
            command.bio
        );
        
        self.service.update_profile(context, &command.user_id, &profile).await
    }
}
