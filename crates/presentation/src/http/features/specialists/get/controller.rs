use axum::{
    Json,
    extract::{ State },
    http::{ StatusCode }
};

use application::features::specialists::queries::get::{ GetSpecialistsQuery };

use crate::http::{ HttpState };

use super::{ GetSpecialistsResponse };

pub async fn get(
    State(state): State<HttpState>
) -> Result<(StatusCode, Json<GetSpecialistsResponse>), StatusCode> {
    let view = state.query_bus.dispatch::<GetSpecialistsQuery>(GetSpecialistsQuery { }).await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let response = view.into();
    
    Ok((StatusCode::OK, Json(response)))
}
