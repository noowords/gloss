use chrono::{ NaiveDate, NaiveTime };
use uuid::{ Uuid };

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Appointment {
    pub id: Uuid,
    pub specialist_id: Uuid,
    pub client_id: Uuid,
    pub date: NaiveDate,
    pub time: NaiveTime,
    pub duration: u32,
    pub status: String
}
