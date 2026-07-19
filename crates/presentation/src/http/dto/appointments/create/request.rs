use serde::{ Deserialize };
use chrono::{ NaiveDate, NaiveTime };
use uuid::{ Uuid };

use application::commands::create_appointment::{ CreateAppointmentCommand };

#[derive(Deserialize)]
pub struct CreateAppointmentRequest {
    pub master_id: Uuid,
    pub client_id: Uuid,
    pub date: NaiveDate,
    pub time: NaiveTime
}

impl From<CreateAppointmentRequest> for CreateAppointmentCommand {
    fn from(req: CreateAppointmentRequest) -> Self {
        Self {
            master_id: req.master_id,
            client_id: req.client_id,
            date: req.date,
            time: req.time
        }
    }
}
