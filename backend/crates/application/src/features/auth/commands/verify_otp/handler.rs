use std::sync::{ Arc };
use async_trait::{ async_trait };

use domain::aggregates::{
    user::{ User },
    user_identity::{ UserIdentity }
};

use crate::contracts::cqrs::command::{ Command, CommandHandler, CommandContext };
use super::{ VerifyOtpCommand, VerifyOtpCommandResult, VerifyOtpCommandService };

pub struct VerifyOtpCommandHandler {
    service: Arc<dyn VerifyOtpCommandService>
}

impl VerifyOtpCommandHandler {
    pub fn build(service: Arc<dyn VerifyOtpCommandService>) -> Self {
        Self { service }
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

        match self.service.get_user_id(
            context,
            &command.provider_type.clone().try_into()?,
            &command.provider_key.clone().into(),
        ).await? {
            Some(user_id) => {
                Ok(VerifyOtpCommandResult {
                    user_id,
                    has_profile: self.service.check_profile_exists(context, &user_id).await?
                })
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

                Ok(VerifyOtpCommandResult {
                    user_id: user.id(),
                    has_profile: false
                })
            }
        }
    }
}
