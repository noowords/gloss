use domain::aggregates::appointment::{ Appointment };

use crate::persistence::mysql::features::users::rows::value_objects::{ MySqlUserIdRow };

use super::value_objects::{ MySqlAppointmentIdRow, MySqlAppointmentDateRow, MySqlAppointmentTimeRow, MySqlAppointmentDurationRow, MySqlAppointmentStatusRow };

#[derive(Debug, Clone, sqlx::FromRow)]
pub struct MySqlAppointmentRow {
    pub id: MySqlAppointmentIdRow,
    pub specialist_id: MySqlUserIdRow,
    pub client_id: MySqlUserIdRow,
    pub date: MySqlAppointmentDateRow,
    pub time: MySqlAppointmentTimeRow,
    pub duration: MySqlAppointmentDurationRow,
    pub status: Option<MySqlAppointmentStatusRow>
}

impl TryFrom<MySqlAppointmentRow> for Appointment {
    type Error = anyhow::Error;

    fn try_from(row: MySqlAppointmentRow) -> Result<Self, Self::Error> {
        Ok(Self::restore(
            row.id.into(),
            row.specialist_id.into(),
            row.client_id.into(),
            row.date.into(),
            row.time.into(),
            row.duration.into(),
            row.status
                .ok_or_else(|| anyhow::anyhow!("Missing appointment status in database"))?
                .try_into()?,
        ))
    }
}

impl From<&Appointment> for MySqlAppointmentRow {
    fn from(entity: &Appointment) -> Self {
        Self {
            id: entity.id().into(),
            specialist_id: entity.specialist_id().into(),
            client_id: entity.client_id().into(),
            date: entity.date().into(),
            time: entity.time().into(),
            duration: entity.duration().into(),
            status: Some(entity.status().into())
        }
    }
}
