use axum::{
    Json,
    extract::{ State },
    http::{ StatusCode }
};

use application::features::account::commands::update_profile::{ UpdateAccountProfileCommand };

use crate::http::{ HttpState, AuthContext };

use super::{ UpdateAccountProfileRequest };

pub async fn update_profile(
    auth_ctx: AuthContext,
    State(state): State<HttpState>,
    Json(payload): Json<UpdateAccountProfileRequest>,
) -> Result<StatusCode, StatusCode> {
    let command = UpdateAccountProfileCommand {
        user_id: auth_ctx.user_id.into(),
        first_name: payload.first_name.into(),
        last_name: payload.last_name.map(|ln| ln.into()),
        avatar_url: payload.avatar_url.map(|url| url.into()),
        bio: payload.bio.map(|bio| bio.into()),
    };

    state.command_bus.dispatch::<UpdateAccountProfileCommand>(command).await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(StatusCode::OK)
}
