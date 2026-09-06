use domain::aggregates::user_identity::value_objects::{ UserIdentityProviderType };

#[derive(Debug, Clone, PartialEq, Eq, sqlx::Type, sqlx::FromRow)]
#[sqlx(transparent)]
pub struct MySqlUserIdentityProviderTypeRow(String);

impl From<MySqlUserIdentityProviderTypeRow> for String {
    fn from(row: MySqlUserIdentityProviderTypeRow) -> Self {
        row.0
    }
}

impl From<String> for MySqlUserIdentityProviderTypeRow {
    fn from(value: String) -> Self {
        Self(value)
    }
}

impl TryFrom<MySqlUserIdentityProviderTypeRow> for UserIdentityProviderType {
    type Error = anyhow::Error;
    
    fn try_from(row: MySqlUserIdentityProviderTypeRow) -> Result<Self, Self::Error> {
        row.0.try_into()
            .map_err(|e| anyhow::anyhow!("Failed to restore Identity Provider Type from DB string: {:?}", e))
    }
}

impl From<UserIdentityProviderType> for MySqlUserIdentityProviderTypeRow {
    fn from(entity: UserIdentityProviderType) -> Self {
        Self(entity.into())
    }
}
