use axum::{ Router, routing::* };

use super::{ HttpState, features::* };

pub fn create_http_router(state: HttpState) -> Router {
    Router::new()
        .route("/auth/otp/send", post(auth::send_otp))
        .route("/auth/otp/verify", post(auth::verify_otp))
        .route("/auth/refresh", post(auth::refresh_tokens))
        .route("/auth/logout", post(auth::logout))
        .route("/account", get(account::get))
        .route("/account/profile",
            post(account::create_profile)
            .get(account::get_profile)
            .put(account::update_profile)
        )
        
        .route("/specialists", get(specialists::get))
        .route("/specialists/{id}", get(specialists::get_by_user_id))
        .route("/specialists/{id}/services", get(specialists::get_services_by_user_id))
        .route("/appointments",
            post(appointments::schedule)
            .get(appointments::get)
        )
        .with_state(state)
}
