use axum::{
    Json,
    extract::{ State },
    http::{ StatusCode },
    response::{ IntoResponse }
};

use crate::application::commands::create_appointment::{ CreateAppointmentCommand };

use super::super::super::{ HttpState };

pub async fn create(
    State(state): State<HttpState>,
    Json(cmd): Json<CreateAppointmentCommand>
) -> impl IntoResponse {
    match state.command_bus.send::<CreateAppointmentCommand, ()>(cmd).await {
        Ok(_) => (StatusCode::CREATED, Json(serde_json::json!({ "message": "Appointment created" }))),
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, Json(serde_json::json!({ "error": e.to_string() })))
    }
}
