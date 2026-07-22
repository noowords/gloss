use chrono::{ NaiveDate, NaiveTime };
use uuid::{ Uuid };

use crate::buses::command_bus::{ Command };
use crate::persistence::{
    command_results::{ ScheduleAppointmentCommandResult },
    command_handlers::{ ScheduleAppointmentCommandHandler },
};

#[derive(Clone)]
pub struct ScheduleAppointmentCommand {
    pub master_id: Uuid,
    pub client_id: Uuid,
    pub date: NaiveDate,
    pub time: NaiveTime,
}

impl Command for ScheduleAppointmentCommand {
    type Result = ScheduleAppointmentCommandResult;
    type Error = anyhow::Error;

    type Handler = ScheduleAppointmentCommandHandler;
}
