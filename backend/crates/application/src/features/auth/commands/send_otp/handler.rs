use std::sync::{ Arc };
use async_trait::{ async_trait };

use chrono::{ Duration, Utc };
use domain::aggregates::auth::otp_challenge::{
    OtpChallenge,
    value_objects::{ OtpChallengeCodeHash, OtpChallengePurpose, OtpChallengeExpiresAt }
};

use crate::contracts::cqrs::command::{ Command, CommandHandler, CommandContext };
use super::{ SendOtpCommand, SendOtpCommandService };

pub struct SendOtpCommandHandler {
    service: Arc<dyn SendOtpCommandService>
}

impl SendOtpCommandHandler {
    pub fn build(service: Arc<dyn SendOtpCommandService>) -> Self {
        Self { service }
    }
}

#[async_trait]
impl CommandHandler<SendOtpCommand> for SendOtpCommandHandler {
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
