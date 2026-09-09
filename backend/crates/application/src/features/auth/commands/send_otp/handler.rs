use std::sync::{ Arc };
use async_trait::{ async_trait };

use domain::aggregates::otp::{ Otp };

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
            &command.provider_key.clone().into()
        ).await?;
        
        let otp = Otp::generate(
            command.provider_type.try_into()?,
            command.provider_key.into()
        );

        self.service.save_otp(context, &otp).await?;
        
        println!(
            "[AUTH] Отправлен код {} на номер {}",
            String::from(otp.code()),
            String::from(otp.provider_key())
        );

        Ok(())
    }
}
