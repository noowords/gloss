use axum::{
    Router,
    routing::{ post, get }
};

use super::{ HttpState, handlers };

pub fn create_router(state: HttpState) -> Router {
    Router::new()
        .route("/health", get(handlers::root::health))
        .route("/auth/register", post(handlers::auth::register))
        .route("/users", get(handlers::users::get))
        .route("/users/{id}", get(handlers::users::get_by_id))
        .route("/users/{id}/profile", get(handlers::users::get_profile_by_id))
        .route("/appointments", post(handlers::appointments::create))
        .with_state(state)
}
