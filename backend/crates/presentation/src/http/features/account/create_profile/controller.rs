use axum::{
    Json,
    extract::{ State },
    http::{ StatusCode }
};

use application::features::account::commands::create_profile::{ CreateAccountProfileCommand };

use crate::http::{ HttpState, AuthContext };

use super::{ CreateAccountProfileRequest };

pub async fn create_profile(
    auth_ctx: AuthContext,
    State(state): State<HttpState>,
    Json(payload): Json<CreateAccountProfileRequest>,
) -> Result<StatusCode, StatusCode> {
    let command = CreateAccountProfileCommand {
        user_id: auth_ctx.user_id.into(),
        first_name: payload.first_name.into(),
        last_name: payload.last_name.map(|ln| ln.into()),
        avatar_url: payload.avatar_url.map(|url| url.into()),
        bio: payload.bio.map(|bio| bio.into()),
    };
    
    state.command_bus.dispatch::<CreateAccountProfileCommand>(command).await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(StatusCode::OK)
}
