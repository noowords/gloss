use domain::aggregates::user_identity::{ UserIdentity };

use crate::persistence::mysql::features::users::rows::value_objects::{ MySqlUserIdRow };

use super::value_objects::{ MySqlUserIdentityIdRow, MySqlUserIdentityProviderTypeRow, MySqlUserIdentityProviderKeyRow, MySqlUserIdentityProviderDataRow };

#[derive(Debug, Clone, sqlx::FromRow)]
pub struct MySqlUserIdentityRow {
    pub id: MySqlUserIdentityIdRow,
    pub user_id: MySqlUserIdRow,
    pub provider_type: MySqlUserIdentityProviderTypeRow,
    pub provider_key: MySqlUserIdentityProviderKeyRow,
    pub provider_data: Option<MySqlUserIdentityProviderDataRow>
}

impl TryFrom<MySqlUserIdentityRow> for UserIdentity {
    type Error = anyhow::Error;

    fn try_from(model: MySqlUserIdentityRow) -> Result<Self, Self::Error> {
        Ok(Self::restore(
            model.id.into(),
            model.user_id.into(),
            model.provider_type.try_into()?,
            model.provider_key.into(),
            model.provider_data
                .map(|pd| pd.try_into())
                .transpose()?
        ))
    }
}

impl TryFrom<&UserIdentity> for MySqlUserIdentityRow {
    type Error = anyhow::Error;
    
    fn try_from(entity: &UserIdentity) -> Result<Self, Self::Error> {
        Ok(Self {
            id: entity.id().into(),
            user_id: entity.user_id().into(),
            provider_type: entity.provider_type().into(),
            provider_key: entity.provider_key().into(),
            provider_data: entity.provider_data()
                .map(|pd| pd.try_into())
                .transpose()?
        })
    }
}
