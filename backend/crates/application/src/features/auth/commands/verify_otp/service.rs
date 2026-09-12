use async_trait::{ async_trait };

use domain::aggregates::{
    user::{
        User,
        value_objects::{ UserId }
    },
    user_identity::{ UserIdentity },
    otp::{
        Otp,
        value_objects::{ OtpProviderType, OtpProviderKey, OtpCode }
    }
};

use crate::contracts::cqrs::command::{ CommandContext };

#[async_trait]
pub trait VerifyOtpCommandService: Send + Sync {
    async fn get_otp(&self, ctx: &mut dyn CommandContext, provider_type: &OtpProviderType, provider_key: &OtpProviderKey, code: &OtpCode) -> Result<Option<Otp>, anyhow::Error>;
    
    async fn remove_otps(&self, ctx: &mut dyn CommandContext, provider_type: &OtpProviderType, provider_key: &OtpProviderKey) -> Result<(), anyhow::Error>;

    async fn get_user_by_otp(&self, ctx: &mut dyn CommandContext, provider_type: &OtpProviderType, provider_key: &OtpProviderKey) -> Result<Option<User>, anyhow::Error>;

    async fn check_profile_exists(&self, ctx: &mut dyn CommandContext, user_id: &UserId) -> Result<bool, anyhow::Error>;

    async fn save_user(&self, ctx: &mut dyn CommandContext, user: &User) -> Result<(), anyhow::Error>;

    async fn save_user_identity(&self, ctx: &mut dyn CommandContext, user_identity: &UserIdentity) -> Result<(), anyhow::Error>;
}
