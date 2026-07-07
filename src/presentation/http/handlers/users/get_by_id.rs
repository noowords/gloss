use axum::{
    Json,
    extract::{ State, Path },
    http::{ StatusCode }
};

use crate::application::queries::get_user_by_id::{ GetUserByIdQuery };

use super::super::super::{
    HttpState,
    dto::users::get_by_id::{ GetUserByIdRequest }
};

pub async fn get_by_id(
    State(state): State<HttpState>,
    Path(req): Path<GetUserByIdRequest>
) -> Result<(StatusCode, Json<serde_json::Value>), StatusCode> {
    let query = req.into();

    state.query_bus.send::<GetUserByIdQuery, ()>(query)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok((StatusCode::OK, Json(serde_json::json!({ "message": "User created" }))))
}
