use axum::{
    Json,
    extract::{ State },
    http::{ StatusCode }
};

use crate::http::{ HttpState };

use super::{ LogoutRequest };

pub async fn logout(
    State(_state): State<HttpState>,
    Json(_payload): Json<LogoutRequest>
) -> Result<StatusCode, StatusCode> {
    Ok(StatusCode::OK)
}
