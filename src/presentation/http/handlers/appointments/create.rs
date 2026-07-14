use axum::{
    Json,
    extract::{ State },
    http::{ StatusCode }
};

use crate::application::commands::create_appointment::{ CreateAppointmentCommand };

use super::super::super::{
    HttpState,
    dto::appointments::create::{ CreateAppointmentRequest }
};

pub async fn create(
    State(state): State<HttpState>,
    Json(req): Json<CreateAppointmentRequest>
) -> Result<StatusCode, StatusCode> {
    state.command_bus.send::<CreateAppointmentCommand, ()>(req.into())
        .await
        .map(|_| StatusCode::CREATED)
        .map_err(|e| {
            eprintln!("[Error] Failed to execute CreateAppointmentCommand: {:?}", e);
            StatusCode::INTERNAL_SERVER_ERROR
        })
}
