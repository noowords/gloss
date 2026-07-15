use axum::{
    Json,
    extract::{ State },
    http::{ StatusCode }
};

use crate::application::queries::users::get::{ GetUsersQuery, GetUsersView };

use super::super::super::{ HttpState };

pub async fn get(
    State(state): State<HttpState>
) -> Result<(StatusCode, Json<GetUsersView>), StatusCode> {
    match state.query_bus.send::<GetUsersQuery, GetUsersView>(GetUsersQuery { }).await {
        Ok(users) => Ok((StatusCode::OK, Json(users))),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR)
    }
}
