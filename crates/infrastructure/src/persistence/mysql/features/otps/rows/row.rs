use domain::aggregates::otp::{ Otp };

use super::value_objects::{ MySqlOtpIdRow, MySqlOtpPhoneRow, MySqlOtpCodeRow };

#[derive(Debug, Clone, sqlx::FromRow)]
pub struct MySqlOtpRow {
    pub id: MySqlOtpIdRow,
    pub phone: MySqlOtpPhoneRow,
    pub code: MySqlOtpCodeRow
}

impl TryFrom<MySqlOtpRow> for Otp {
    type Error = anyhow::Error;

    fn try_from(model: MySqlOtpRow) -> Result<Self, Self::Error> {
        Ok(Self::restore(
            model.id.into(),
            model.phone.try_into()?,
            model.code.into()
        ))
    }
}

impl From<&Otp> for MySqlOtpRow {
    fn from(entity: &Otp) -> Self {
        Self {
            id: entity.id().into(),
            phone: entity.phone().into(),
            code: entity.code().into()
        }
    }
}
