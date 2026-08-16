use axum::{ Router, routing::{ post, get } };

use super::{ HttpState, features::* };

pub fn create_http_router(state: HttpState) -> Router {
    Router::new()
        .route("/health", get(root::health))
        .route("/auth/register", post(auth::register))
        .route("/users", get(users::get))
        .route("/users/{id}", get(users::get_by_id))
        .route("/users/{id}/profile", get(users::get_profile_by_id))
        .route("/specialists", get(specialists::get))
        .route("/specialists/{id}", get(specialists::get_by_user_id))
        .route("/specialists/{id}/services", get(specialists::get_services_by_user_id))
        .route("/appointments", post(appointments::schedule))
        .route("/appointments", get(appointments::get))
        .with_state(state)
}
