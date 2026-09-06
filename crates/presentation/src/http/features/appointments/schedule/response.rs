use serde::{ Serialize };

use application::features::appointments::commands::schedule::{ ScheduleAppointmentCommandResult };

#[derive(Serialize)]
pub struct ScheduleAppointmentResponse { }

impl From<ScheduleAppointmentCommandResult> for ScheduleAppointmentResponse {
    fn from(_result: ScheduleAppointmentCommandResult) -> Self {
        Self { }
    }
}
