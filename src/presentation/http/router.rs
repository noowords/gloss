use axum::{
    Router,
    routing::{ post, get }
};

use super::{ HttpState, handlers };

pub fn create_router(state: HttpState) -> Router {
    Router::new()
        .route("/health", get(handlers::root::health))
        .route("/users", post(handlers::users::create))
        .route("/appointments", post(handlers::appointments::create))
        .with_state(state)
}
