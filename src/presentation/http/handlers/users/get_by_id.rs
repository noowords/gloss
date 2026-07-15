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
    Path(payload): Path<GetUserByIdRequest>
) -> Result<(StatusCode, Json<GetUserByIdView>), StatusCode> {
    match state.query_bus.send::<GetUserByIdQuery, Option<GetUserByIdView>>(payload.into()).await {
        Ok(Some(user)) => Ok((StatusCode::OK, Json(user))),
        Ok(None) => Err(StatusCode::NOT_FOUND),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR)
    }
}

