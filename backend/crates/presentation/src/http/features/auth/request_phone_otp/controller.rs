use axum::{
    Json,
    extract::{ State },
    http::{ StatusCode }
};

use application::features::auth::commands::request_otp::{ RequestOtpCommand };

use crate::http::{ HttpState };

use super::{ RequestPhoneOtpRequest, RequestPhoneOtpResponse };

pub async fn request_phone_otp(
    State(state): State<HttpState>,
    Json(payload): Json<RequestPhoneOtpRequest>
) -> Result<(StatusCode, Json<RequestPhoneOtpResponse>), StatusCode> {
    let result = state.command_bus.dispatch::<RequestOtpCommand>(payload.into()).await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    
    let response = result.into();

    Ok((StatusCode::OK, Json(response)))
}
