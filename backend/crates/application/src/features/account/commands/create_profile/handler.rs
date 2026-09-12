use std::sync::{ Arc };
use async_trait::{ async_trait };

use domain::aggregates::profile::{ Profile };

use crate::contracts::cqrs::command::{ Command, CommandHandler, CommandContext };
use super::{ CreateAccountProfileCommand, CreateAccountProfileCommandService };

pub struct CreateAccountProfileCommandHandler {
    service: Arc<dyn CreateAccountProfileCommandService>
}

impl CreateAccountProfileCommandHandler {
    pub fn build(service: Arc<dyn CreateAccountProfileCommandService>) -> Self {
        Self { service }
    }
}

#[async_trait]
impl CommandHandler<CreateAccountProfileCommand> for CreateAccountProfileCommandHandler {
    async fn handle(&self, context: &mut dyn CommandContext, command: CreateAccountProfileCommand) -> Result<
        <CreateAccountProfileCommand as Command>::Result,
        <CreateAccountProfileCommand as Command>::Error
    > {
        let profile = Profile::create(
            command.user_id,
            command.first_name,
            command.last_name,
            command.avatar_url,
            command.bio
        );
        
        self.service.create_profile(context, &profile).await
    }
}
