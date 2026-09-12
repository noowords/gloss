use std::sync::{ Arc };
use async_trait::{ async_trait };

use crate::contracts::{
    TokenService,
    cqrs::command::{ Command, CommandHandler, CommandContext }
};
use super::{ RefreshTokensCommand, RefreshTokensCommandResult, RefreshTokensCommandService };

pub struct RefreshTokensCommandHandler {
    service: Arc<dyn RefreshTokensCommandService>,
    token_service: Arc<dyn TokenService>
}

impl RefreshTokensCommandHandler {
    pub fn build(service: Arc<dyn RefreshTokensCommandService>, token_service: Arc<dyn TokenService>) -> Self {
        Self { service, token_service }
    }
}

#[async_trait]
impl CommandHandler<RefreshTokensCommand> for RefreshTokensCommandHandler {
    async fn handle(&self, context: &mut dyn CommandContext, command: RefreshTokensCommand) -> Result<
        <RefreshTokensCommand as Command>::Result,
        <RefreshTokensCommand as Command>::Error
    > {
        let user_id = self.token_service.verify_refresh_token(&command.refresh_token)?;
        let (user_role, has_profile) = match self.service.get_user_by_id(
            context,
            &user_id
        ).await? {
            Some(user) => (
                user.role(),
                self.service.check_profile_exists(context, &user_id).await?
            ),
            None => anyhow::bail!("User not found")
        };
        
        Ok(RefreshTokensCommandResult {
            access_token: self.token_service.generate_access_token(user_id, user_role)?,
            refresh_token: self.token_service.generate_refresh_token(user_id)?,
            user_id,
            has_profile
        })
    }
}
