use async_trait::{ async_trait };
use std::sync::{ Arc };

use domain::aggregates::{
    users::user::{ User },
    users::user_provider::{ UserProvider },
    users::user_role::value_objects::{ UserRoleName }
};

use crate::common::{
    TokenService,
    cqrs::command::{ Command, CommandHandler, CommandContext }
};
use super::{ VerifyOtpCommand, VerifyOtpCommandResult, VerifyOtpCommandService };

pub struct VerifyOtpCommandHandler<S, T> {
    service: S,
    token_service: Arc<T>
}

impl<S, T> VerifyOtpCommandHandler<S, T>
where
    S: VerifyOtpCommandService,
    T: TokenService
{
    pub fn build(service: S, token_service: Arc<T>) -> Self {
        Self { service, token_service }
    }
}

#[async_trait]
impl<S, T> CommandHandler<VerifyOtpCommand> for VerifyOtpCommandHandler<S, T>
where
    S: VerifyOtpCommandService,
    T: TokenService
{
    async fn handle(&self, context: &mut dyn CommandContext, command: VerifyOtpCommand) -> Result<
        <VerifyOtpCommand as Command>::Result,
        <VerifyOtpCommand as Command>::Error
    > {
        let otp = self.service.get_otp(
            context,
            &command.provider_type.clone().try_into()?,
            &command.provider_key.clone().try_into()?,
            &"login".try_into()?,
            &command.code.clone().into_bytes().try_into()?
        ).await?;

        if otp.is_none() {
            return Err(anyhow::anyhow!("OTP not found").into());
        }

        self.service.remove_otps(
            context,
            &command.provider_type.clone().try_into()?,
            &command.provider_key.clone().try_into()?,
            &"login".try_into()?,
        ).await?;

        let (user, has_profile) = match self.service.get_user_by_otp(
            context,
            &command.provider_type.clone().try_into()?,
            &command.provider_key.clone().try_into()?,
        ).await? {
            Some(user) => {
                let has_profile = self.service.check_profile_exists(context, &user.id()).await?;
                
                (user, has_profile)
            }
            None => {
                let user = User::create()?;
                
                self.service.save_user(context, &user).await?;
                
                let user_identity = UserProvider::create(
                    user.id(),
                    command.provider_type.clone().try_into()?,
                    command.provider_key.clone().try_into()?,
                    None
                )?;
                
                self.service.save_user_provider(context, &user_identity).await?;

                (user, false)
            }
        };

        Ok(VerifyOtpCommandResult {
            access_token: self.token_service.generate_access_token(user.id(), UserRoleName::try_from("client")?)?,
            refresh_token: self.token_service.generate_refresh_token(user.id())?,
            user_id: user.id(),
            has_profile
        })
    }
}
