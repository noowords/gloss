use async_trait::{ async_trait };

use domain::aggregates::auth::otp_challenge::{
    OtpChallenge,
    value_objects::{ OtpChallengeProvider, OtpChallengeSubject }
};

use crate::common::cqrs::command::{ CommandContext };

#[async_trait]
pub trait SendOtpCommandService: Send + Sync + 'static {
    async fn save_otp(&self, ctx: &mut dyn CommandContext, otp: &OtpChallenge) -> Result<(), anyhow::Error>;

    async fn delete_old_otps(&self, ctx: &mut dyn CommandContext, provider: &OtpChallengeProvider, subject: &OtpChallengeSubject) -> Result<(), anyhow::Error>;
}
