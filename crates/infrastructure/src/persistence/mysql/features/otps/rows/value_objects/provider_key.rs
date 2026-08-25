use domain::aggregates::otp::value_objects::{ OtpProviderKey };

#[derive(Debug, Clone, PartialEq, Eq, sqlx::Type)]
#[sqlx(transparent)]
pub struct MySqlOtpProviderKeyRow(String);

impl From<MySqlOtpProviderKeyRow> for String {
    fn from(row: MySqlOtpProviderKeyRow) -> Self {
        row.0
    }
}

impl From<String> for MySqlOtpProviderKeyRow {
    fn from(value: String) -> Self {
        Self(value)
    }
}

impl From<MySqlOtpProviderKeyRow> for OtpProviderKey {
    fn from(row: MySqlOtpProviderKeyRow) -> Self {
        row.0.into()
    }
}

impl From<OtpProviderKey> for MySqlOtpProviderKeyRow {
    fn from(entity: OtpProviderKey) -> Self {
        Self(entity.into())
    }
}
