use axum::{
    Json,
    extract::{ State, Path },
    http::{ StatusCode }
};

use application::features::users::queries::get_by_id::{ GetUserByIdQuery };

use crate::http::{ HttpState };

use super::{ GetUserByIdRequest, GetUserByIdResponse };

pub async fn get_by_id(
    State(state): State<HttpState>,
    Path(payload): Path<GetUserByIdRequest>
) -> Result<(StatusCode, Json<GetUserByIdResponse>), StatusCode> {
    let view = state.query_bus.dispatch::<GetUserByIdQuery>(payload.into()).await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let response = view.try_into()
        .map_err(|_| StatusCode::NOT_FOUND)?;

    Ok((StatusCode::OK, Json(response)))
}
