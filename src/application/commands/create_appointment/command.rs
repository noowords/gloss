use chrono::{ NaiveDate, NaiveTime };
use uuid::{ Uuid };
use serde::{ Deserialize };

#[derive(Deserialize)]
pub struct CreateAppointmentCommand {
    pub master_id: Uuid,
    pub client_id: Uuid,
    pub date: NaiveDate,
    pub time: NaiveTime
}
