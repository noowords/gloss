use axum::{
    Json,
    extract::{ State },
    http::{ StatusCode }
};

use application::features::account::queries::get_profile::{ GetAccountProfileQuery };

use crate::http::{ HttpState, AuthContext };

use super::{ GetAccountProfileResponse };

pub async fn get_profile(
    auth_ctx: AuthContext,
    State(state): State<HttpState>
) -> Result<(StatusCode, Json<GetAccountProfileResponse>), StatusCode> {
    let query = GetAccountProfileQuery { user_id: auth_ctx.user_id.into() };

    let view = state.query_bus.dispatch::<GetAccountProfileQuery>(query).await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let response = view.try_into()
        .map_err(|_| StatusCode::NOT_FOUND)?;

    Ok((StatusCode::OK, Json(response)))
}
