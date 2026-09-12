use axum::{
    Json,
    extract::{ State },
    http::{ StatusCode }
};

use application::features::account::queries::get::{ GetAccountQuery };

use crate::http::{ HttpState, AuthContext };

use super::{ GetAccountResponse };

pub async fn get(
    auth_ctx: AuthContext,
    State(state): State<HttpState>
) -> Result<(StatusCode, Json<GetAccountResponse>), StatusCode> {
    let query = GetAccountQuery { id: auth_ctx.user_id.into() };

    let view = state.query_bus.dispatch::<GetAccountQuery>(query).await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let response = view.try_into()
        .map_err(|_| StatusCode::NOT_FOUND)?;

    Ok((StatusCode::OK, Json(response)))
}
