use domain::aggregates::user_identity::value_objects::{ OtpProviderType };

#[derive(Debug, Clone, PartialEq, Eq, sqlx::Type)]
#[sqlx(transparent)]
pub struct MySqlOtpProviderTypeRow(String);

impl From<MySqlOtpProviderTypeRow> for String {
    fn from(row: MySqlOtpProviderTypeRow) -> Self {
        row.0
    }
}

impl From<String> for MySqlOtpProviderTypeRow {
    fn from(value: String) -> Self {
        Self(value)
    }
}

impl TryFrom<MySqlOtpProviderTypeRow> for OtpProviderType {
    type Error = anyhow::Error;
    
    fn try_from(row: MySqlOtpProviderTypeRow) -> Result<Self, Self::Error> {
        row.0.try_into()
            .map_err(|e| anyhow::anyhow!("Failed to restore Identity Provider Type from DB string: {:?}", e))
    }
}

impl From<OtpProviderType> for MySqlOtpProviderTypeRow {
    fn from(entity: OtpProviderType) -> Self {
        Self(entity.into())
    }
}
