use axum::{
    Json,
    extract::{ State },
    http::{ StatusCode }
};

use application::features::users::commands::register::{ RegisterUserCommand };

use crate::http::{ HttpState };

use super::{ RegisterUserRequest };

pub async fn register(
    State(state): State<HttpState>,
    Json(payload): Json<RegisterUserRequest>,
) -> Result<StatusCode, StatusCode> {
    state.command_bus.dispatch::<RegisterUserCommand>(payload.into()).await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(StatusCode::OK)
}
