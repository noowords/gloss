use sqlx::types::{ JsonValue };

use domain::aggregates::user_identity::value_objects::{ UserIdentityProviderData };

#[derive(Debug, Clone, PartialEq, Eq, sqlx::Type, sqlx::FromRow)]
#[sqlx(transparent)]
pub struct MySqlUserIdentityProviderDataRow(JsonValue);

impl From<MySqlUserIdentityProviderDataRow> for JsonValue {
    fn from(row: MySqlUserIdentityProviderDataRow) -> Self {
        row.0
    }
}

impl From<JsonValue> for MySqlUserIdentityProviderDataRow {
    fn from(value: JsonValue) -> Self {
        Self(value)
    }
}

impl TryFrom<MySqlUserIdentityProviderDataRow> for UserIdentityProviderData {
    type Error = anyhow::Error;
    
    fn try_from(row: MySqlUserIdentityProviderDataRow) -> Result<Self, Self::Error> {
        Ok(
            row.0.to_string().try_into()
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
