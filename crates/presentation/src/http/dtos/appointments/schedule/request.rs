use serde::{ Deserialize };
use chrono::{ NaiveDate, NaiveTime };
use uuid::{ Uuid };

use application::features::appointments::commands::schedule_appointment::{ ScheduleAppointmentCommand };

#[derive(Deserialize)]
pub struct ScheduleAppointmentRequest {
    pub master_id: Uuid,
    pub client_id: Uuid,
    pub date: NaiveDate,
    pub time: NaiveTime
}

impl From<ScheduleAppointmentRequest> for ScheduleAppointmentCommand {
    fn from(req: ScheduleAppointmentRequest) -> Self {
        Self {
            master_id: req.master_id,
            client_id: req.client_id,
            date: req.date,
            time: req.time
        }
    }
}
