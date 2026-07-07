use axum::{
    Json,
    extract::{ State },
    http::{ StatusCode }
};

use crate::application::commands::create_user::{ CreateUserCommand };

use super::super::super::{
    HttpState,
    dto::users::create::{ CreateUserRequest }
};

pub async fn create(
    State(state): State<HttpState>,
    Json(req): Json<CreateUserRequest>
) -> Result<StatusCode, StatusCode> {
    let command = req.into();

    state.command_bus.send::<CreateUserCommand, ()>(command)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(StatusCode::CREATED)
}
