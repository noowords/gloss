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
    state.query_bus.send::<GetUsersQuery, GetUsersView>(GetUsersQuery {})
        .await
        .map(|users| (StatusCode::OK, Json(users)))
        .map_err(|e| {
            eprintln!("[Error] Failed to execute GetUsersQuery: {:?}", e);
            StatusCode::INTERNAL_SERVER_ERROR
        })
}
