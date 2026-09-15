use domain::aggregates::auth::otp_challenge::{ OtpChallenge };

use super::value_objects::{
    MySqlOtpChallengeAttemptsRow,
    MySqlOtpChallengeCodeHashRow,
    MySqlOtpChallengeConsumedAtRow,
    MySqlOtpChallengeExpiresAtRow,
    MySqlOtpChallengeIdRow,
    MySqlOtpChallengeProviderRow,
    MySqlOtpChallengePurposeRow,
    MySqlOtpChallengeSubjectRow,
    MySqlOtpChallengeVerifiedAtRow
};

#[derive(Debug, Clone, sqlx::FromRow)]
pub struct MySqlOtpChallengeRow {
    pub id: MySqlOtpChallengeIdRow,
    pub provider: MySqlOtpChallengeProviderRow,
    pub subject: MySqlOtpChallengeSubjectRow,
    pub purpose: MySqlOtpChallengePurposeRow,
    pub code_hash: MySqlOtpChallengeCodeHashRow,
    pub attempts: MySqlOtpChallengeAttemptsRow,
    pub expires_at: MySqlOtpChallengeExpiresAtRow,
    pub verified_at: Option<MySqlOtpChallengeVerifiedAtRow>,
    pub consumed_at: Option<MySqlOtpChallengeConsumedAtRow>
}

impl TryFrom<MySqlOtpChallengeRow> for OtpChallenge {
    type Error = anyhow::Error;

    fn try_from(row: MySqlOtpChallengeRow) -> Result<Self, Self::Error> {
        Self::restore(
            uuid::Uuid::from(row.id).into(),
            String::from(row.provider).try_into()?,
            String::from(row.subject).try_into()?,
            String::from(row.purpose).try_into()?,
            Vec::<u8>::from(row.code_hash).try_into()?,
            u16::from(row.attempts).into(),
            chrono::NaiveDateTime::from(row.expires_at).into(),
            row.verified_at.map(|value| chrono::NaiveDateTime::from(value).into()),
            row.consumed_at.map(|value| chrono::NaiveDateTime::from(value).into())
        )
    }
}

impl From<&OtpChallenge> for MySqlOtpChallengeRow {
    fn from(entity: &OtpChallenge) -> Self {
        Self {
            id: uuid::Uuid::from(entity.id()).into(),
            provider: String::from(entity.provider()).into(),
            subject: String::from(entity.subject()).into(),
            purpose: String::from(entity.purpose()).into(),
            code_hash: Vec::<u8>::from(entity.code_hash()).into(),
            attempts: u16::from(entity.attempts()).into(),
            expires_at: chrono::NaiveDateTime::from(entity.expires_at()).into(),
            verified_at: entity.verified_at().map(|value| chrono::NaiveDateTime::from(value).into()),
            consumed_at: entity.consumed_at().map(|value| chrono::NaiveDateTime::from(value).into())
        }
    }
}
