use axum::{
    Router,
    routing::{ post, get }
};

use super::{ HttpState, controllers };

pub fn create_http_router(state: HttpState) -> Router {
    Router::new()
        .route("/health", get(controllers::health))
        .route("/auth/register", post(controllers::auth::register))
        .route("/users", get(controllers::users::get))
        .route("/users/{id}", get(controllers::users::get_by_id))
        .route("/users/{id}/profile", get(controllers::users::get_profile_by_id))
        .route("/appointments", post(controllers::appointments::schedule))
        .with_state(state)
}
