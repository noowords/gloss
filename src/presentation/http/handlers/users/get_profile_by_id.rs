use axum::{
    Json,
    extract::{ State, Path },
    http::{ StatusCode }
};

use crate::application::queries::users::get_profile_by_id::{ GetUserProfileByIdQuery, GetUserProfileByIdView };

use super::super::super::{
    HttpState,
    dto::users::get_profile_by_id::{ GetUserProfileByIdRequest }
};

pub async fn get_profile_by_id(
    State(state): State<HttpState>,
    Path(req): Path<GetUserProfileByIdRequest>
) -> Result<(StatusCode, Json<GetUserProfileByIdView>), StatusCode> {
    state.query_bus.send::<GetUserProfileByIdQuery, Option<GetUserProfileByIdView>>(req.into())
        .await
        .map_err(|e| {
            eprintln!("[Error] Failed to execute GetUserProfileByIdQuery: {:?}", e);
            StatusCode::INTERNAL_SERVER_ERROR
        })?
        .ok_or(StatusCode::NOT_FOUND)
        .map(|profile| (StatusCode::OK, Json(profile)))
}
