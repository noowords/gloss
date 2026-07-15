use chrono::{ NaiveDate, NaiveTime };
use uuid::{ Uuid };
use serde::{ Deserialize };

use super::super::super::common::commands::{ Command }; 

use super::{ CreateAppointmentHandler };

#[derive(Deserialize)]
pub struct CreateAppointmentCommand {
    pub master_id: Uuid,
    pub client_id: Uuid,
    pub date: NaiveDate,
    pub time: NaiveTime
}

impl Command for CreateAppointmentCommand {
    type Output = ();
    type Error = anyhow::Error;

    type Handler = CreateAppointmentHandler;
}

