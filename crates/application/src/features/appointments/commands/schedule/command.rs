use chrono::{ NaiveDate, NaiveTime };
use uuid::{ Uuid };

use crate::contracts::cqrs::command::{ Command };
use super::{ ScheduleAppointmentCommandHandler };

#[derive(Clone)]
pub struct ScheduleAppointmentCommand {
    pub specialist_id: Uuid,
    pub client_id: Uuid,
    pub date: NaiveDate,
    pub time: NaiveTime,
    pub service_ids: Vec<Uuid>
}

impl Command for ScheduleAppointmentCommand {
    type Error = anyhow::Error;

    type Handler = ScheduleAppointmentCommandHandler;
}
