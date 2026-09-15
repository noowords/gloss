use domain::aggregates::auth::auth_session::{ AuthSession };

use super::value_objects::{
    MySqlAuthSessionExpiresAtRow,
    MySqlAuthSessionIdRow,
    MySqlAuthSessionIpAddressRow,
    MySqlAuthSessionLastUsedAtRow,
    MySqlAuthSessionRefreshTokenHashRow,
    MySqlAuthSessionRevokedAtRow,
    MySqlAuthSessionUserAgentRow,
    MySqlAuthSessionUserIdRow
};

#[derive(Debug, Clone, sqlx::FromRow)]
pub struct MySqlAuthSessionRow {
    pub id: MySqlAuthSessionIdRow,
    pub user_id: MySqlAuthSessionUserIdRow,
    pub refresh_token_hash: MySqlAuthSessionRefreshTokenHashRow,
    pub user_agent: Option<MySqlAuthSessionUserAgentRow>,
    pub ip_address: Option<MySqlAuthSessionIpAddressRow>,
    pub expires_at: MySqlAuthSessionExpiresAtRow,
    pub last_used_at: Option<MySqlAuthSessionLastUsedAtRow>,
    pub revoked_at: Option<MySqlAuthSessionRevokedAtRow>
}

impl TryFrom<MySqlAuthSessionRow> for AuthSession {
    type Error = anyhow::Error;

    fn try_from(row: MySqlAuthSessionRow) -> Result<Self, Self::Error> {
        AuthSession::restore(
            uuid::Uuid::from(row.id).into(),
            uuid::Uuid::from(row.user_id).into(),
            Vec::<u8>::from(row.refresh_token_hash).try_into()?,
            row.user_agent.map(|value| String::from(value).try_into()).transpose()?,
            row.ip_address.map(|value| Vec::<u8>::from(value).try_into()).transpose()?,
            chrono::NaiveDateTime::from(row.expires_at).into(),
            row.last_used_at.map(|value| chrono::NaiveDateTime::from(value).into()),
            row.revoked_at.map(|value| chrono::NaiveDateTime::from(value).into())
        )
    }
}

impl From<&AuthSession> for MySqlAuthSessionRow {
    fn from(entity: &AuthSession) -> Self {
        Self {
            id: uuid::Uuid::from(entity.id()).into(),
            user_id: uuid::Uuid::from(entity.user_id()).into(),
            refresh_token_hash: Vec::<u8>::from(entity.refresh_token_hash()).into(),
            user_agent: entity.user_agent().map(|value| String::from(value).into()),
            ip_address: entity.ip_address().map(|value| Vec::<u8>::from(value).into()),
            expires_at: chrono::NaiveDateTime::from(entity.expires_at()).into(),
            last_used_at: entity.last_used_at().map(|value| chrono::NaiveDateTime::from(value).into()),
            revoked_at: entity.revoked_at().map(|value| chrono::NaiveDateTime::from(value).into())
        }
    }
}
