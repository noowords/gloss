use axum::{
    Json,
    extract::{ State },
    http::{ StatusCode }
};

use application::features::auth::commands::send_otp::{ SendOtpCommand };

use crate::http::{ HttpState };

use super::{ SendOtpRequest };

pub async fn send_otp(
    State(state): State<HttpState>,
    Json(payload): Json<SendOtpRequest>
) -> Result<StatusCode, StatusCode> {
    state.command_bus.dispatch::<SendOtpCommand>(payload.into()).await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(StatusCode::OK)
}
