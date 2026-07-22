use axum::{
    Json,
    extract::{ State, Path },
    http::{ StatusCode }
};

use application::projections::queries::{ GetUserByIdQuery };

use super::super::super::{
    HttpState,
    dto::users::get_by_id::{ GetUserByIdRequest, GetUserByIdResponse }
};

pub async fn get_by_id(
    State(state): State<HttpState>,
    Path(payload): Path<GetUserByIdRequest>
) -> Result<(StatusCode, Json<GetUserByIdResponse>), StatusCode> {
    match state.query_bus.send::<GetUserByIdQuery>(payload.into()).await {
        Ok(Ok(output)) if output.value().is_some() => Ok((StatusCode::OK, Json(output.into()))),
        Ok(Ok(_)) => Err(StatusCode::NOT_FOUND),
        Ok(Err(_)) => Err(StatusCode::INTERNAL_SERVER_ERROR),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR)
    }
}
