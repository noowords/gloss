use axum::{
    Json,
    extract::{ State, Path },
    http::{ StatusCode }
};

use application::features::users::queries::get_profile_by_id::{ GetUserProfileByIdQuery };

use crate::http::{ HttpState };

use super::{ GetUserProfileByIdRequest, GetUserProfileByIdResponse };

pub async fn get_profile_by_id(
    State(state): State<HttpState>,
    Path(payload): Path<GetUserProfileByIdRequest>
) -> Result<(StatusCode, Json<GetUserProfileByIdResponse>), StatusCode> {
    let view = state.query_bus.dispatch::<GetUserProfileByIdQuery>(payload.into()).await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let response = view.try_into()
        .map_err(|_| StatusCode::NOT_FOUND)?;

    Ok((StatusCode::OK, Json(response)))
}
