use axum::{
    Json,
    extract::{ State },
    http::{ StatusCode }
};

use application::features::appointments::commands::schedule::{ ScheduleAppointmentCommand };

use crate::http::{ HttpState };

use super::{ ScheduleAppointmentRequest, ScheduleAppointmentResponse };

pub async fn schedule(
    State(state): State<HttpState>,
    Json(payload): Json<ScheduleAppointmentRequest>
) -> Result<(StatusCode, Json<ScheduleAppointmentResponse>), StatusCode> {
    let result = state.command_bus.dispatch::<ScheduleAppointmentCommand>(payload.into()).await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    
    let response = result.into();

    Ok((StatusCode::OK, Json(response)))
}
