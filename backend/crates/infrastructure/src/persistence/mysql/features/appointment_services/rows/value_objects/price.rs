use bigdecimal::{ BigDecimal };

use domain::aggregates::appointments::appointment_service::value_objects::{ AppointmentServicePriceSnapshot };

#[derive(Debug, Clone, PartialEq, Eq, sqlx::Type, sqlx::FromRow)]
#[sqlx(transparent)]
pub struct MySqlAppointmentServicePriceSnapshotRow(BigDecimal);

impl From<MySqlAppointmentServicePriceSnapshotRow> for BigDecimal {
    fn from(row: MySqlAppointmentServicePriceSnapshotRow) -> Self {
        row.0
    }
}

impl From<BigDecimal> for MySqlAppointmentServicePriceSnapshotRow {
    fn from(value: BigDecimal) -> Self {
        Self(value)
    }
}

impl TryFrom<MySqlAppointmentServicePriceSnapshotRow> for AppointmentServicePriceSnapshot {
    type Error = anyhow::Error;

    fn try_from(row: MySqlAppointmentServicePriceSnapshotRow) -> Result<Self, Self::Error> {
        row.0.try_into()
    }
}

impl From<AppointmentServicePriceSnapshot> for MySqlAppointmentServicePriceSnapshotRow {
    fn from(entity: AppointmentServicePriceSnapshot) -> Self {
        Self(entity.into())
    }
}
