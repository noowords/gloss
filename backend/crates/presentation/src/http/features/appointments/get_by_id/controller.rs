use axum::{
    Json,
    extract::{ State, Path },
    http::{ StatusCode }
};

use application::features::appointments::queries::get_by_id::{ GetAppointmentByIdQuery };

use crate::http::{ HttpState };

use super::{ GetAppointmentByIdRequest, GetAppointmentByIdResponse };

pub async fn get_by_id(
    State(state): State<HttpState>,
    Path(payload): Path<GetAppointmentByIdRequest>
) -> Result<(StatusCode, Json<GetAppointmentByIdResponse>), StatusCode> {
    let view = state.query_bus.dispatch::<GetAppointmentByIdQuery>(payload.into()).await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let response = view.try_into()
        .map_err(|_| StatusCode::NOT_FOUND)?;

    Ok((StatusCode::OK, Json(response)))
}
