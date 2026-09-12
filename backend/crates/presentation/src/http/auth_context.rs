use uuid::{ Uuid };
use axum::{
    extract::{ FromRequestParts, FromRef },
    http::{
        StatusCode,
        request::Parts
    }
};

use super::{ HttpState };

pub struct AuthContext {
    pub user_id: Uuid,
    pub role: String
}

impl<S> FromRequestParts<S> for AuthContext
where
    HttpState: FromRef<S>,
    S: Send + Sync,
{
    type Rejection = (StatusCode, &'static str);

    async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        let app_state = HttpState::from_ref(state);
        let token_service = app_state.token_service.clone();

        let auth_header = parts.headers
            .get(axum::http::header::AUTHORIZATION)
            .and_then(|value| value.to_str().ok())
            .ok_or((StatusCode::UNAUTHORIZED, "Missing Authorization header"))?;

        if !auth_header.starts_with("Bearer ") {
            return Err((StatusCode::UNAUTHORIZED, "Invalid token format"));
        }

        let token = &auth_header[7..];

        let (user_id, role) = token_service
            .verify_access_token(token)
            .map_err(|_| (StatusCode::UNAUTHORIZED, "Invalid token"))?;

        Ok(AuthContext { user_id: user_id.into(), role: role.into() })
    }
}
