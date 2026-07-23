use axum::{
    Json,
    extract::{ State, Path },
    http::{ StatusCode }
};

use application::features::users::queries::get_user_profile_by_id::{ GetUserProfileByIdQuery };

use super::super::super::{
    HttpState,
    dtos::users::get_profile_by_id::{ GetUserProfileByIdRequest, GetUserProfileByIdResponse }
};

pub async fn get_profile_by_id(
    State(state): State<HttpState>,
    Path(payload): Path<GetUserProfileByIdRequest>
) -> Result<(StatusCode, Json<GetUserProfileByIdResponse>), StatusCode> {
    match state.query_bus.send::<GetUserProfileByIdQuery>(payload.into()).await {
        Ok(Ok(output)) if output.value().is_some() => Ok((StatusCode::OK, Json(output.into()))),
        Ok(Ok(_)) => Err(StatusCode::NOT_FOUND),
        Ok(Err(_)) => Err(StatusCode::INTERNAL_SERVER_ERROR),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}
