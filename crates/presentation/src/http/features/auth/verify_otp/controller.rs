use axum::{
    Json,
    extract::{ State },
    http::{ StatusCode }
};

use application::features::auth::commands::verify_otp::{ VerifyOtpCommand };

use crate::http::{ HttpState };

use super::{ VerifyOtpRequest, VerifyOtpResponse };

pub async fn verify_otp(
    State(state): State<HttpState>,
    Json(payload): Json<VerifyOtpRequest>
) -> Result<(StatusCode, Json<VerifyOtpResponse>), StatusCode> {
    let result = state.command_bus.dispatch::<VerifyOtpCommand>(payload.into()).await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let response = result.into();

    Ok((StatusCode::OK, Json(response)))
}
