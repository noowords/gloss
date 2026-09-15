use domain::aggregates::users::user_provider::{ UserProvider };

use super::value_objects::{
    MySqlUserProviderIdRow,
    MySqlUserProviderProviderRow,
    MySqlUserProviderSubjectRow,
    MySqlUserProviderUserIdRow,
    MySqlUserProviderVerifiedAtRow
};

#[derive(Debug, Clone, sqlx::FromRow)]
pub struct MySqlUserProviderRow {
    pub id: MySqlUserProviderIdRow,
    pub user_id: MySqlUserProviderUserIdRow,
    pub provider: MySqlUserProviderProviderRow,
    pub subject: MySqlUserProviderSubjectRow,
    pub verified_at: Option<MySqlUserProviderVerifiedAtRow>
}

impl TryFrom<MySqlUserProviderRow> for UserProvider {
    type Error = anyhow::Error;

    fn try_from(row: MySqlUserProviderRow) -> Result<Self, Self::Error> {
        Self::restore(
            uuid::Uuid::from(row.id).into(),
            uuid::Uuid::from(row.user_id).into(),
            String::from(row.provider).try_into()?,
            String::from(row.subject).try_into()?,
            row.verified_at.map(|value| chrono::NaiveDateTime::from(value).into())
        )
    }
}

impl From<&UserProvider> for MySqlUserProviderRow {
    fn from(entity: &UserProvider) -> Self {
        Self {
            id: uuid::Uuid::from(entity.id()).into(),
            user_id: uuid::Uuid::from(entity.user_id()).into(),
            provider: String::from(entity.provider()).into(),
            subject: String::from(entity.subject()).into(),
            verified_at: entity.verified_at().map(|value| chrono::NaiveDateTime::from(value).into())
        }
    }
}
