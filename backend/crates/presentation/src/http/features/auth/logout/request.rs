use serde::{ Deserialize };

#[derive(Deserialize)]
pub struct LogoutRequest {
    pub refresh_token: String
}
