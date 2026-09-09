use axum::{
    Json,
    extract::{ State },
    http::{ StatusCode }
};

use application::features::users::queries::get::{ GetUsersQuery };

use crate::http::{ HttpState };

use super::{ GetUsersResponse };

pub async fn get(
    State(state): State<HttpState>
) -> Result<(StatusCode, Json<GetUsersResponse>), StatusCode> {
    let view = state.query_bus.dispatch::<GetUsersQuery>(GetUsersQuery { }).await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let response = view.into();
    
    Ok((StatusCode::OK, Json(response)))
}
