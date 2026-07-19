use axum::{
    Json,
    extract::{ State },
    http::{ StatusCode }
};

use application::commands::create_appointment::{ CreateAppointmentCommand };

use super::super::super::{
    HttpState,
    dto::appointments::create::{ CreateAppointmentRequest }
};

pub async fn create(
    State(state): State<HttpState>,
    Json(payload): Json<CreateAppointmentRequest>
) -> Result<StatusCode, StatusCode> {
    match state.command_bus.send::<CreateAppointmentCommand>(payload.into()).await {
        Ok(_) => Ok(StatusCode::CREATED),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR)
    }
}
