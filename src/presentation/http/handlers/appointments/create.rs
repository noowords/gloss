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
    let command = req.into();

    state.command_bus.send::<CreateAppointmentCommand, ()>(command)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(StatusCode::CREATED)
}
