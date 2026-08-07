use domain::aggregates::otp::value_objects::{ OtpPhone };

#[derive(Debug, Clone, PartialEq, Eq, sqlx::Type)]
#[sqlx(transparent)]
pub struct MySqlOtpPhoneRow(String);

impl TryFrom<MySqlOtpPhoneRow> for OtpPhone {
    type Error = anyhow::Error;

    fn try_from(row: MySqlOtpPhoneRow) -> Result<Self, Self::Error> {
        row.0.try_into()
    }
}

impl From<OtpPhone> for MySqlOtpPhoneRow {
    fn from(entity: OtpPhone) -> Self {
        Self(entity.into())
    }
}
