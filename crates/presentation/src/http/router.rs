use axum::{ Router, routing::{ post, get } };

use super::{ HttpState, features::* };

pub fn create_http_router(state: HttpState) -> Router {
    Router::new()
        .route("/health", get(root::health))
        .route("/auth/register", post(auth::register))
        .route("/users", get(users::get))
        .route("/users/{id}", get(users::get_by_id))
        .route("/users/{id}/profile", get(users::get_profile_by_id))
        .route("/appointments", post(appointments::schedule))
        .with_state(state)
}
