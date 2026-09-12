use axum::{
    Json,
    extract::{ State },
    http::{ StatusCode }
};

use application::features::auth::commands::refresh_tokens::{ RefreshTokensCommand };

use crate::http::{ HttpState };

use super::{ RefreshTokensRequest, RefreshTokensResponse };

pub async fn refresh_tokens(
    State(state): State<HttpState>,
    Json(payload): Json<RefreshTokensRequest>
) -> Result<(StatusCode, Json<RefreshTokensResponse>), StatusCode> {
    let result = state.command_bus.dispatch::<RefreshTokensCommand>(payload.into()).await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let response = result.into();

    Ok((StatusCode::OK, Json(response)))
}
