use std::sync::{ Arc };
use async_trait::{ async_trait };

use domain::aggregates::{
    user::{ User },
    user_identity::{ UserIdentity }
};

use crate::contracts::{
    TokenService,
    cqrs::command::{ Command, CommandHandler, CommandContext }
};
use super::{ VerifyOtpCommand, VerifyOtpCommandResult, VerifyOtpCommandService };

pub struct VerifyOtpCommandHandler {
    service: Arc<dyn VerifyOtpCommandService>,
    token_service: Arc<dyn TokenService>
}

impl VerifyOtpCommandHandler {
    pub fn build(service: Arc<dyn VerifyOtpCommandService>, token_service: Arc<dyn TokenService>) -> Self {
        Self { service, token_service }
    }
}

#[async_trait]
impl CommandHandler<VerifyOtpCommand> for VerifyOtpCommandHandler {
    async fn handle(&self, context: &mut dyn CommandContext, command: VerifyOtpCommand) -> Result<
        <VerifyOtpCommand as Command>::Result,
        <VerifyOtpCommand as Command>::Error
    > {
        let otp = self.service.get_otp(
            context,
            &command.provider_type.clone().try_into()?,
            &command.provider_key.clone().into(),
            &command.code.clone().into()
        ).await?;

        if otp.is_none() {
            return Err(anyhow::anyhow!("OTP not found").into());
        }

        self.service.remove_otps(
            context,
            &command.provider_type.clone().try_into()?,
            &command.provider_key.clone().into(),
        ).await?;

        let (user, has_profile) = match self.service.get_user_by_otp(
            context,
            &command.provider_type.clone().try_into()?,
            &command.provider_key.clone().into(),
        ).await? {
            Some(user) => {
                let has_profile = self.service.check_profile_exists(context, &user.id()).await?;
                
                (user, has_profile)
            }
            None => {
                let user = User::create();
                
                self.service.save_user(context, &user).await?;
                
                let user_identity = UserIdentity::create(
                    user.id(),
                    command.provider_type.clone().try_into()?,
                    command.provider_key.clone().into(),
                    None
                );
                
                self.service.save_user_identity(context, &user_identity).await?;

                (user, false)
            }
        };

        Ok(VerifyOtpCommandResult {
            access_token: self.token_service.generate_access_token(user.id(), user.role())?,
            refresh_token: self.token_service.generate_refresh_token(user.id())?,
            user_id: user.id(),
            has_profile
        })
    }
}
