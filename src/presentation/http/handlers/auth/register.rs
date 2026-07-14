use axum::{Json, extract::State, http::StatusCode};

use crate::application::commands::register_user::RegisterUserCommand;

use super::super::super::{HttpState, dto::auth::register::RegisterUserRequest};

pub async fn register(
    State(state): State<HttpState>,
    Json(req): Json<RegisterUserRequest>,
) -> Result<StatusCode, StatusCode> {
    state
        .command_bus
        .send::<RegisterUserCommand, ()>(req.into())
        .await
        .map(|_| StatusCode::CREATED)
        .map_err(|e| {
            eprintln!("[Error] Failed to execute RegisterUserCommand: {:?}", e);
            StatusCode::INTERNAL_SERVER_ERROR
        })
}
