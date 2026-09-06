use application::features::appointments::queries::get_by_id::dtos::{ Appointment };

use crate::persistence::mysql::features::{
    users::rows::value_objects::{ MySqlUserIdRow },
    appointments::rows::value_objects::{ MySqlAppointmentIdRow, MySqlAppointmentDateRow, MySqlAppointmentTimeRow, MySqlAppointmentDurationRow, MySqlAppointmentStatusRow }
};

#[derive(Debug, Clone, sqlx::FromRow)]
pub struct MySqlAppointmentRow {
    pub id: MySqlAppointmentIdRow,
    pub specialist_id: MySqlUserIdRow,
    pub client_id: MySqlUserIdRow,
    pub date: MySqlAppointmentDateRow,
    pub time: MySqlAppointmentTimeRow,
    pub duration: MySqlAppointmentDurationRow,
    pub status: MySqlAppointmentStatusRow
}

impl From<MySqlAppointmentRow> for Appointment {
    fn from(row: MySqlAppointmentRow) -> Self {
        Self {
            id: row.id.into(),
            specialist_id: row.specialist_id.into(),
            client_id: row.client_id.into(),
            date: row.date.into(),
            time: row.time.into(),
            duration: row.duration.into(),
            status: row.status.into()
        }
    }
}

impl From<&Appointment> for MySqlAppointmentRow {
    fn from(entity: &Appointment) -> Self {
        Self {
            id: entity.id.into(),
            specialist_id: entity.specialist_id.into(),
            client_id: entity.client_id.into(),
            date: entity.date.into(),
            time: entity.time.into(),
            duration: entity.duration.into(),
            status: entity.status.clone().into()
        }
    }
}
