use axum::{
    Json,
    extract::{ State },
    http::{ StatusCode }
};

use application::features::queries::get_users::{ GetUsersQuery };

use super::super::super::{
    HttpState,
    dtos::users::get::{ GetUsersResponse }
};

pub async fn get(
    State(state): State<HttpState>
) -> Result<(StatusCode, Json<GetUsersResponse>), StatusCode> {
    match state.query_bus.send::<GetUsersQuery>(GetUsersQuery { }).await {
        Ok(Ok(output)) => Ok((StatusCode::OK, Json(output.into()))),
        Ok(Err(_)) => Err(StatusCode::INTERNAL_SERVER_ERROR),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR)
    }
}
