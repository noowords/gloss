use axum::{
    Json,
    extract::{ State },
    http::{ StatusCode },
    response::{ IntoResponse }
};

use crate::application::commands::create_user::{ CreateUserCommand };

use super::super::super::{ HttpState };

pub async fn create(
    State(state): State<HttpState>,
    Json(cmd): Json<CreateUserCommand>
) -> impl IntoResponse {
    match state.command_bus.send::<CreateUserCommand, ()>(cmd).await {
        Ok(_) => (StatusCode::CREATED, Json(serde_json::json!({ "message": "User created" }))),
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, Json(serde_json::json!({ "error": e.to_string() })))
    }
}
