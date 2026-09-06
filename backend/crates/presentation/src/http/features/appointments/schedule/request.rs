use serde::{ Deserialize };
use chrono::{ NaiveDate, NaiveTime };
use uuid::{ Uuid };

use application::features::appointments::commands::schedule::{ ScheduleAppointmentCommand };

#[derive(Deserialize)]
pub struct ScheduleAppointmentRequest {
    pub specialist_id: Uuid,
    pub client_id: Uuid,
    pub date: NaiveDate,
    pub time: NaiveTime,
    pub service_ids: Vec<Uuid>,
}

impl From<ScheduleAppointmentRequest> for ScheduleAppointmentCommand {
    fn from(req: ScheduleAppointmentRequest) -> Self {
        Self {
            specialist_id: req.specialist_id,
            client_id: req.client_id,
            date: req.date,
            time: req.time,
            service_ids: req.service_ids
        }
    }
}
