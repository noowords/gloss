use chrono::{NaiveDate, NaiveTime};
use serde::Deserialize;
use uuid::Uuid;

use super::super::super::buses::command_bus::Command;

use super::CreateAppointmentHandler;

#[derive(Deserialize)]
pub struct CreateAppointmentCommand {
    pub master_id: Uuid,
    pub client_id: Uuid,
    pub date: NaiveDate,
    pub time: NaiveTime,
}

impl Command for CreateAppointmentCommand {
    type Output = ();
    type Error = anyhow::Error;

    type Handler = CreateAppointmentHandler;
}
