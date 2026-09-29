use async_trait::{ async_trait };

use chrono::{ Duration, Utc };
use domain::aggregates::auth::otp_challenge::{
    OtpChallenge,
    value_objects::{ OtpChallengeCodeHash, OtpChallengePurpose, OtpChallengeExpiresAt }
};

use crate::common::cqrs::command::{ Command, CommandHandler, CommandContext };
use super::{ SendOtpCommand, SendOtpCommandService };

pub struct SendOtpCommandHandler<S> {
    service: S
}

impl<S> SendOtpCommandHandler<S>
where
    S: SendOtpCommandService
{
    pub fn build(service: S) -> Self {
        Self { service }
    }
}

#[async_trait]
impl<S> CommandHandler<SendOtpCommand> for SendOtpCommandHandler<S>
where
    S: SendOtpCommandService
{
    async fn handle(&self, context: &mut dyn CommandContext, command: SendOtpCommand) -> Result<
        <SendOtpCommand as Command>::Result,
        <SendOtpCommand as Command>::Error
    > {
        self.service.delete_old_otps(
            context,
            &command.provider_type.clone().try_into()?,
            &command.provider_key.clone().try_into()?
        ).await?;
        
        let otp = OtpChallenge::create(
            command.provider_type.try_into()?,
            command.provider_key.try_into()?,
            OtpChallengePurpose::try_from("login")?,
            OtpChallengeCodeHash::try_from(Vec::new())?,
            OtpChallengeExpiresAt::from((Utc::now() + Duration::minutes(10)).naive_utc()),
            None,
            None
        )?;

        self.service.save_otp(context, &otp).await?;
        
        Ok(())
    }
}
