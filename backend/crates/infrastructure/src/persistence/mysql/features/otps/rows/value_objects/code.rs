use domain::aggregates::otp::value_objects::{ OtpCode };

#[derive(Debug, Clone, PartialEq, Eq, sqlx::Type, sqlx::FromRow)]
#[sqlx(transparent)]
pub struct MySqlOtpCodeRow(String);

impl From<MySqlOtpCodeRow> for String {
    fn from(row: MySqlOtpCodeRow) -> Self {
        row.0
    }
}

impl From<String> for MySqlOtpCodeRow {
    fn from(value: String) -> Self {
        Self(value)
    }
}

impl From<MySqlOtpCodeRow> for OtpCode {
    fn from(row: MySqlOtpCodeRow) -> Self {
        row.0.into()
    }
}

impl From<OtpCode> for MySqlOtpCodeRow {
    fn from(entity: OtpCode) -> Self {
        Self(entity.into())
    }
}
