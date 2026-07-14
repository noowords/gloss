use axum::{
    Json,
    extract::{ State, Path },
    http::{ StatusCode }
};

use crate::application::queries::users::get_by_id::{ GetUserByIdQuery, GetUserByIdView };

use super::super::super::{
    HttpState,
    dto::users::get_by_id::{ GetUserByIdRequest }
};

pub async fn get_by_id(
    State(state): State<HttpState>,
    Path(req): Path<GetUserByIdRequest>
) -> Result<(StatusCode, Json<GetUserByIdView>), StatusCode> {
    state.query_bus.send::<GetUserByIdQuery, Option<GetUserByIdView>>(req.into())
        .await
        .map_err(|e| {
            eprintln!("[Error] Failed to execute GetUserByIdQuery: {:?}", e);
            StatusCode::INTERNAL_SERVER_ERROR
        })?
        .ok_or(StatusCode::NOT_FOUND)
        .map(|user| (StatusCode::OK, Json(user)))
}
