use domain::aggregates::user_identity::value_objects::{ UserIdentityProviderType };

#[derive(Debug, Clone, PartialEq, Eq, sqlx::Type)]
#[sqlx(transparent)]
pub struct MySqlUserIdentityProviderTypeRow(String);

impl TryFrom<MySqlUserIdentityProviderTypeRow> for UserIdentityProviderType {
    type Error = anyhow::Error;
    
    fn try_from(model: MySqlUserIdentityProviderTypeRow) -> Result<Self, Self::Error> {
        model.0.try_into()
            .map_err(|e| anyhow::anyhow!("Failed to restore Identity Provider Type from DB string: {:?}", e))
    }
}

impl From<UserIdentityProviderType> for MySqlUserIdentityProviderTypeRow {
    fn from(entity: UserIdentityProviderType) -> Self {
        Self(entity.into())
    }
}
