use chrono::{ NaiveDate, NaiveTime };
use uuid::{ Uuid };
use serde::{ Serialize };

use application::features::appointments::queries::get::dtos::{ Appointment };

#[derive(Serialize)]
pub struct HttpAppointmentDto {
    pub id: Uuid,
    pub specialist_id: Uuid,
    pub client_id: Uuid,
    pub date: NaiveDate,
    pub time: NaiveTime,
    pub duration: u32,
    pub status: String
}

impl From<Appointment> for HttpAppointmentDto {
    fn from(entity: Appointment) -> Self {
        Self {
            id: entity.id,
            specialist_id: entity.specialist_id,
            client_id: entity.client_id,
            date: entity.date,
            time: entity.time,
            duration: entity.duration,
            status: entity.status.clone()
        }
    }
}
