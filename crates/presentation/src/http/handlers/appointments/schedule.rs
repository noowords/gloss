use axum::{
    Json,
    extract::{ State },
    http::{ StatusCode }
};

use application::features::commands::schedule_appointment::{ ScheduleAppointmentCommand };

use super::super::super::{
    HttpState,
    dto::appointments::schedule::{ ScheduleAppointmentRequest }
};

pub async fn schedule(
    State(state): State<HttpState>,
    Json(payload): Json<ScheduleAppointmentRequest>
) -> Result<StatusCode, StatusCode> {
    match state.command_bus.send::<ScheduleAppointmentCommand>(payload.into()).await {
        Ok(_) => Ok(StatusCode::CREATED),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR)
    }
}
