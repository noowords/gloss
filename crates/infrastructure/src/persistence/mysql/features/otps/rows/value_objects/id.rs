use uuid::{ Uuid };

use domain::aggregates::otp::value_objects::{ OtpId };

#[derive(Debug, Copy, Clone, PartialEq, Eq, sqlx::Type, sqlx::FromRow)]
#[sqlx(transparent)]
pub struct MySqlOtpIdRow(Uuid);

impl From<MySqlOtpIdRow> for Uuid {
    fn from(row: MySqlOtpIdRow) -> Self {
        row.0
    }
}

impl From<Uuid> for MySqlOtpIdRow {
    fn from(value: Uuid) -> Self {
        Self(value)
    }
}

impl From<MySqlOtpIdRow> for OtpId {
    fn from(row: MySqlOtpIdRow) -> Self {
        row.0.into()
    }
}

impl From<OtpId> for MySqlOtpIdRow {
    fn from(entity: OtpId) -> Self {
        Self(entity.into())
    }
}
