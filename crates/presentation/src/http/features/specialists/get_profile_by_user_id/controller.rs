use axum::{
    Json,
    extract::{ State, Path },
    http::{ StatusCode }
};

use application::features::specialists::queries::get_services_by_user_id::{ GetSpecialistServicesByUserIdQuery };

use crate::http::{ HttpState };

use super::{ GetSpecialistProfileByUserIdRequest, GetSpecialistProfileByUserIdResponse };

pub async fn get_profile_by_user_id(
    State(state): State<HttpState>,
    Path(payload): Path<GetSpecialistProfileByUserIdRequest>
) -> Result<(StatusCode, Json<GetSpecialistProfileByUserIdResponse>), StatusCode> {
    let view = state.query_bus.dispatch::<GetSpecialistServicesByUserIdQuery>(payload.into()).await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let response = view.try_into()
        .map_err(|_| StatusCode::NOT_FOUND)?;

    Ok((StatusCode::OK, Json(response)))
}
