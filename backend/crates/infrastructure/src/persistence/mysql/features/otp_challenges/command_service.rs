use async_trait::async_trait;
use uuid::Uuid;
use application::contracts::cqrs::command::CommandContext;
use application::features::auth::commands::{
    send_otp::SendOtpCommandService,
    verify_otp::VerifyOtpCommandService
};
use domain::aggregates::{
    auth::otp_challenge::{ OtpChallenge, value_objects::{ OtpChallengeProvider, OtpChallengeSubject, OtpChallengePurpose, OtpChallengeCodeHash } },
    users::{ user::{ User, value_objects::UserId }, user_provider::UserProvider }
};
use crate::persistence::mysql::{
    contracts::cqrs::command::MySqlCommandContext,
    features::{ otp_challenges::rows::MySqlOtpChallengeRow, user_providers::rows::MySqlUserProviderRow, users::rows::MySqlUserRow }
};

#[derive(Default)]
pub struct MySqlOtpCommandService;

fn tx(context: &mut dyn CommandContext) -> Result<&mut sqlx::Transaction<'static, sqlx::MySql>, anyhow::Error> {
    context.as_any_mut()
        .downcast_mut::<MySqlCommandContext>()
        .map(|context| context.tx_mut())
        .ok_or_else(|| anyhow::anyhow!("Invalid CommandContext"))
}

#[async_trait]
impl SendOtpCommandService for MySqlOtpCommandService {
    async fn save_otp(&self, context: &mut dyn CommandContext, otp: &OtpChallenge) -> Result<(), anyhow::Error> {
        let tx = tx(context)?;
        let row: MySqlOtpChallengeRow = otp.into();
        sqlx::query("INSERT INTO otp_challenges (id, provider, subject, purpose, code_hash, attempts, expires_at, verified_at, consumed_at) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?)")
            .bind(row.id).bind(row.provider).bind(row.subject).bind(row.purpose).bind(row.code_hash)
            .bind(row.attempts).bind(row.expires_at).bind(row.verified_at).bind(row.consumed_at)
            .execute(&mut **tx).await?;
        Ok(())
    }

    async fn delete_old_otps(&self, context: &mut dyn CommandContext, provider: &OtpChallengeProvider, subject: &OtpChallengeSubject) -> Result<(), anyhow::Error> {
        let tx = tx(context)?;
        sqlx::query("DELETE FROM otp_challenges WHERE provider = ? AND subject = ? AND purpose = 'login'")
            .bind(String::from(provider.clone())).bind(String::from(subject.clone())).execute(&mut **tx).await?;
        Ok(())
    }
}

#[async_trait]
impl VerifyOtpCommandService for MySqlOtpCommandService {
    async fn get_otp(&self, context: &mut dyn CommandContext, provider: &OtpChallengeProvider, subject: &OtpChallengeSubject, purpose: &OtpChallengePurpose, code_hash: &OtpChallengeCodeHash) -> Result<Option<OtpChallenge>, anyhow::Error> {
        let tx = tx(context)?;
        let row: Option<MySqlOtpChallengeRow> = sqlx::query_as("SELECT id, provider, subject, purpose, code_hash, attempts, expires_at, verified_at, consumed_at FROM otp_challenges WHERE provider = ? AND subject = ? AND purpose = ? AND code_hash = ? LIMIT 1")
            .bind(String::from(provider.clone())).bind(String::from(subject.clone())).bind(String::from(purpose.clone())).bind(Vec::<u8>::from(code_hash.clone()))
            .fetch_optional(&mut **tx).await?;
        row.map(TryInto::try_into).transpose()
    }

    async fn remove_otps(&self, context: &mut dyn CommandContext, provider: &OtpChallengeProvider, subject: &OtpChallengeSubject, purpose: &OtpChallengePurpose) -> Result<(), anyhow::Error> {
        let tx = tx(context)?;
        sqlx::query("DELETE FROM otp_challenges WHERE provider = ? AND subject = ? AND purpose = ?")
            .bind(String::from(provider.clone())).bind(String::from(subject.clone())).bind(String::from(purpose.clone())).execute(&mut **tx).await?;
        Ok(())
    }

    async fn get_user_by_otp(&self, context: &mut dyn CommandContext, provider: &OtpChallengeProvider, subject: &OtpChallengeSubject) -> Result<Option<User>, anyhow::Error> {
        let tx = tx(context)?;
        let row: Option<MySqlUserRow> = sqlx::query_as("SELECT u.id, u.status AS status FROM user_providers up INNER JOIN users u ON up.user_id = u.id WHERE up.provider = ? AND up.subject = ? LIMIT 1")
            .bind(String::from(provider.clone())).bind(String::from(subject.clone())).fetch_optional(&mut **tx).await?;
        row.map(TryInto::try_into).transpose()
    }

    async fn check_profile_exists(&self, context: &mut dyn CommandContext, user_id: &UserId) -> Result<bool, anyhow::Error> {
        let tx = tx(context)?;
        let row: Option<(Uuid,)> = sqlx::query_as("SELECT user_id FROM profiles WHERE user_id = ? LIMIT 1").bind(uuid::Uuid::from(*user_id)).fetch_optional(&mut **tx).await?;
        Ok(row.is_some())
    }

    async fn save_user(&self, context: &mut dyn CommandContext, user: &User) -> Result<(), anyhow::Error> {
        let tx = tx(context)?;
        let row: MySqlUserRow = user.into();
        sqlx::query("INSERT INTO users (id, status) VALUES (?, ?)").bind(row.id).bind(row.status).execute(&mut **tx).await?;
        Ok(())
    }

    async fn save_user_provider(&self, context: &mut dyn CommandContext, user_provider: &UserProvider) -> Result<(), anyhow::Error> {
        let tx = tx(context)?;
        let row: MySqlUserProviderRow = user_provider.into();
        sqlx::query("INSERT INTO user_providers (id, user_id, provider, subject, verified_at) VALUES (?, ?, ?, ?, ?)")
            .bind(row.id).bind(row.user_id).bind(row.provider).bind(row.subject).bind(row.verified_at).execute(&mut **tx).await?;
        Ok(())
    }
}
