use domain::aggregates::otp::{ Otp };

use super::value_objects::{ MySqlOtpIdRow, MySqlOtpProviderTypeRow, MySqlOtpProviderKeyRow, MySqlOtpCodeRow };

#[derive(Debug, Clone, sqlx::FromRow)]
pub struct MySqlOtpRow {
    pub id: MySqlOtpIdRow,
    pub provider_type: MySqlOtpProviderTypeRow,
    pub provider_key: MySqlOtpProviderKeyRow,
    pub code: MySqlOtpCodeRow
}

impl TryFrom<MySqlOtpRow> for Otp {
    type Error = anyhow::Error;

    fn try_from(row: MySqlOtpRow) -> Result<Self, Self::Error> {
        Ok(Self::restore(
            row.id.into(),
            row.provider_type.try_into()?,
            row.provider_key.into(),
            row.code.into()
        ))
    }
}

impl From<&Otp> for MySqlOtpRow {
    fn from(entity: &Otp) -> Self {
        Self {
            id: entity.id().into(),
            provider_type: entity.provider_type().into(),
            provider_key: entity.provider_key().into(),
            code: entity.code().into()
        }
    }
}
