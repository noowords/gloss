use bigdecimal::{ BigDecimal };

use domain::aggregates::appointment_service::value_objects::{ AppointmentServiceLockedPrice };

#[derive(Debug, Clone, PartialEq, Eq, sqlx::Type, sqlx::FromRow)]
#[sqlx(transparent)]
pub struct MySqlAppointmentLockedPriceRow(BigDecimal);

impl From<MySqlAppointmentLockedPriceRow> for BigDecimal {
    fn from(row: MySqlAppointmentLockedPriceRow) -> Self {
        row.0
    }
}

impl From<BigDecimal> for MySqlAppointmentLockedPriceRow {
    fn from(value: BigDecimal) -> Self {
        Self(value)
    }
}

impl TryFrom<MySqlAppointmentLockedPriceRow> for AppointmentServiceLockedPrice {
    type Error = anyhow::Error;

    fn try_from(row: MySqlAppointmentLockedPriceRow) -> Result<Self, Self::Error> {
        row.0.try_into()
    }
}

impl From<AppointmentServiceLockedPrice> for MySqlAppointmentLockedPriceRow {
    fn from(entity: AppointmentServiceLockedPrice) -> Self {
        Self(entity.into())
    }
}
