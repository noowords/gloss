use domain::aggregates::otp::value_objects::{ OtpCode };

#[derive(Debug, Clone, PartialEq, Eq, sqlx::Type)]
#[sqlx(transparent)]
pub struct MySqlOtpCodeRow(String);

impl From<MySqlOtpCodeRow> for OtpCode {
    fn from(model: MySqlOtpCodeRow) -> Self {
        model.0.into()
    }
}

impl From<OtpCode> for MySqlOtpCodeRow {
    fn from(entity: OtpCode) -> Self {
        Self(entity.into())
    }
}
