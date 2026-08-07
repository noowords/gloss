use axum::{
    Json,
    extract::{ State },
    http::{ StatusCode }
};

use application::features::appointments::commands::schedule::{ ScheduleAppointmentCommand };

use crate::http::{ HttpState };

use super::{ ScheduleAppointmentRequest };

pub async fn schedule(
    State(state): State<HttpState>,
    Json(payload): Json<ScheduleAppointmentRequest>
) -> Result<StatusCode, StatusCode> {
    state.command_bus.dispatch::<ScheduleAppointmentCommand>(payload.into()).await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(StatusCode::OK)
}
