use async_trait::{ async_trait };

use domain::aggregates::otp::{
    Otp,
    value_objects::{ OtpProviderType, OtpProviderKey }
};

use crate::contracts::cqrs::command::{ CommandContext };

#[async_trait]
pub trait RequestOtpCommandService: Send + Sync {
    async fn save_otp(&self, ctx: &mut dyn CommandContext, otp: &Otp) -> Result<(), anyhow::Error>;

    async fn delete_old_otps(&self, ctx: &mut dyn CommandContext, provider_type: &OtpProviderType, provider_key: &OtpProviderKey) -> Result<(), anyhow::Error>;
}
