use bigdecimal::{ BigDecimal };

use domain::aggregates::appointment_service::value_objects::{ AppointmentServiceLockedPrice };

#[derive(Debug, Clone, PartialEq, Eq, sqlx::Type)]
#[sqlx(transparent)]
pub struct MySqlAppointmentLockedPriceRow(BigDecimal);

impl TryFrom<MySqlAppointmentLockedPriceRow> for AppointmentServiceLockedPrice {
    type Error = anyhow::Error;
    
    fn try_from(model: MySqlAppointmentLockedPriceRow) -> Result<Self, Self::Error> {
        model.0.try_into()
    }
}

impl From<AppointmentServiceLockedPrice> for MySqlAppointmentLockedPriceRow {
    fn from(entity: AppointmentServiceLockedPrice) -> Self {
        Self(entity.into())
    }
}
