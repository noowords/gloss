use sqlx::types::{ JsonValue };

use domain::aggregates::user_identity::value_objects::{ UserIdentityProviderData };

#[derive(Debug, Clone, PartialEq, Eq, sqlx::Type)]
#[sqlx(transparent)]
pub struct MySqlUserIdentityProviderDataRow(JsonValue);

impl TryFrom<MySqlUserIdentityProviderDataRow> for UserIdentityProviderData {
    type Error = anyhow::Error;
    
    fn try_from(model: MySqlUserIdentityProviderDataRow) -> Result<Self, Self::Error> {
        Ok(
            model.0.to_string().try_into()
                .map_err(|e| anyhow::anyhow!("Failed to restore Domain identity from DB JSON: {}", e))?
        )
    }
}

impl TryFrom<UserIdentityProviderData> for MySqlUserIdentityProviderDataRow {
    type Error = anyhow::Error;

    fn try_from(entity: UserIdentityProviderData) -> Result<Self, Self::Error> {
        Ok(Self(
            String::from(entity).try_into()
                .map_err(|e| anyhow::anyhow!("Failed to restore Domain identity from DB JSON: {}", e))?
        ))
    }
}
