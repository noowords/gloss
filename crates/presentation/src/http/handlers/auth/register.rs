use axum::{
    Json,
    extract::{ State },
    http::{ StatusCode }
};

use application::persistence::commands::{ RegisterUserCommand };

use super::super::super::{
    HttpState,
    dto::auth::register::{ RegisterUserRequest }
};

pub async fn register(
    State(state): State<HttpState>,
    Json(payload): Json<RegisterUserRequest>,
) -> Result<StatusCode, StatusCode> {
    match state.command_bus.send::<RegisterUserCommand>(payload.into()).await {
        Ok(_) => Ok(StatusCode::CREATED),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR)
    }
}
