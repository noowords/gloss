use chrono::{ DateTime, Utc };

use domain::aggregates::otp::value_objects::{ OtpExpiresAt };

#[derive(Debug, Clone, PartialEq, Eq, sqlx::Type, sqlx::FromRow)]
#[sqlx(transparent)]
pub struct MySqlOtpExpiresAtRow(DateTime<Utc>);

impl From<MySqlOtpExpiresAtRow> for DateTime<Utc> {
    fn from(row: MySqlOtpExpiresAtRow) -> Self {
        row.0
    }
}

impl From<DateTime<Utc>> for MySqlOtpExpiresAtRow {
    fn from(value: DateTime<Utc>) -> Self {
        Self(value)
    }
}

impl From<MySqlOtpExpiresAtRow> for OtpExpiresAt {
    fn from(row: MySqlOtpExpiresAtRow) -> Self {
        row.0.into()
    }
}

impl From<OtpExpiresAt> for MySqlOtpExpiresAtRow {
    fn from(entity: OtpExpiresAt) -> Self {
        Self(entity.into())
    }
}
