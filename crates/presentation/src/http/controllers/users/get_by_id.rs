use axum::{
    Json,
    extract::{ State, Path },
    http::{ StatusCode }
};

use application::features::users::queries::get_user_by_id::{ GetUserByIdQuery };

use super::super::super::{
    HttpState,
    dtos::users::get_by_id::{ GetUserByIdRequest, GetUserByIdResponse }
};

pub async fn get_by_id(
    State(state): State<HttpState>,
    Path(payload): Path<GetUserByIdRequest>
) -> Result<(StatusCode, Json<GetUserByIdResponse>), StatusCode> {
    match state.query_bus.dispatch::<GetUserByIdQuery>(payload.into()).await {
        Ok(Ok(output)) if output.value().is_some() => Ok((StatusCode::OK, Json(output.into()))),
        Ok(Ok(_)) => Err(StatusCode::NOT_FOUND),
        Ok(Err(_)) => Err(StatusCode::INTERNAL_SERVER_ERROR),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR)
    }
}
