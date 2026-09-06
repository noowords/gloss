use axum::{
    Json,
    extract::{ State },
    http::{ StatusCode }
};

use application::features::appointments::queries::get::{ GetAppointmentsQuery };

use crate::http::{ HttpState };

use super::{ GetAppointmentsResponse };

pub async fn get(
    State(state): State<HttpState>
) -> Result<(StatusCode, Json<GetAppointmentsResponse>), StatusCode> {
    let view = state.query_bus.dispatch::<GetAppointmentsQuery>(GetAppointmentsQuery { }).await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let response = view.into();
    
    Ok((StatusCode::OK, Json(response)))
}
