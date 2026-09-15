use domain::aggregates::appointments::appointment_service::{ AppointmentService };

use super::value_objects::{
    MySqlAppointmentServiceAppointmentIdRow,
    MySqlAppointmentServiceDurationMinutesSnapshotRow,
    MySqlAppointmentServicePriceSnapshotRow,
    MySqlAppointmentServiceRoleRow,
    MySqlAppointmentServiceServiceIdRow,
    MySqlAppointmentServiceNameSnapshotRow
};

#[derive(Debug, Clone, sqlx::FromRow)]
pub struct MySqlAppointmentServiceRow {
    pub appointment_id: MySqlAppointmentServiceAppointmentIdRow,
    pub service_id: MySqlAppointmentServiceServiceIdRow,
    pub role: MySqlAppointmentServiceRoleRow,
    pub service_name_snapshot: MySqlAppointmentServiceNameSnapshotRow,
    pub price_snapshot: MySqlAppointmentServicePriceSnapshotRow,
    pub duration_minutes_snapshot: MySqlAppointmentServiceDurationMinutesSnapshotRow
}

impl TryFrom<MySqlAppointmentServiceRow> for AppointmentService {
    type Error = anyhow::Error;

    fn try_from(row: MySqlAppointmentServiceRow) -> Result<Self, Self::Error> {
        Self::restore(
            uuid::Uuid::from(row.appointment_id).into(),
            uuid::Uuid::from(row.service_id).into(),
            String::from(row.role).try_into()?,
            String::from(row.service_name_snapshot).try_into()?,
            bigdecimal::BigDecimal::from(row.price_snapshot).try_into()?,
            u16::from(row.duration_minutes_snapshot).try_into()?
        )
    }
}

impl From<&AppointmentService> for MySqlAppointmentServiceRow {
    fn from(entity: &AppointmentService) -> Self {
        Self {
            appointment_id: uuid::Uuid::from(entity.appointment_id()).into(),
            service_id: uuid::Uuid::from(entity.service_id()).into(),
            role: String::from(entity.role()).into(),
            service_name_snapshot: String::from(entity.service_name_snapshot()).into(),
            price_snapshot: bigdecimal::BigDecimal::from(entity.price_snapshot()).into(),
            duration_minutes_snapshot: u16::from(entity.duration_minutes_snapshot()).into()
        }
    }
}
