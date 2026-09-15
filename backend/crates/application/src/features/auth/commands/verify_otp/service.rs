use async_trait::{ async_trait };

use domain::aggregates::{
    users::user::{
        User,
        value_objects::{ UserId }
    },
    users::user_provider::{ UserProvider },
    auth::otp_challenge::{
        OtpChallenge,
        value_objects::{ OtpChallengeProvider, OtpChallengeSubject, OtpChallengePurpose, OtpChallengeCodeHash }
    }
};

use crate::contracts::cqrs::command::{ CommandContext };

#[async_trait]
pub trait VerifyOtpCommandService: Send + Sync {
    async fn get_otp(&self, ctx: &mut dyn CommandContext, provider: &OtpChallengeProvider, subject: &OtpChallengeSubject, purpose: &OtpChallengePurpose, code_hash: &OtpChallengeCodeHash) -> Result<Option<OtpChallenge>, anyhow::Error>;
    
    async fn remove_otps(&self, ctx: &mut dyn CommandContext, provider: &OtpChallengeProvider, subject: &OtpChallengeSubject, purpose: &OtpChallengePurpose) -> Result<(), anyhow::Error>;

    async fn get_user_by_otp(&self, ctx: &mut dyn CommandContext, provider: &OtpChallengeProvider, subject: &OtpChallengeSubject) -> Result<Option<User>, anyhow::Error>;

    async fn check_profile_exists(&self, ctx: &mut dyn CommandContext, user_id: &UserId) -> Result<bool, anyhow::Error>;

    async fn save_user(&self, ctx: &mut dyn CommandContext, user: &User) -> Result<(), anyhow::Error>;

    async fn save_user_provider(&self, ctx: &mut dyn CommandContext, user_provider: &UserProvider) -> Result<(), anyhow::Error>;
}
