use serde::{ Deserialize };

#[derive(Debug, Clone, Deserialize)]
pub struct UpdateAccountProfileRequest {
    pub first_name: String,
    pub last_name: Option<String>,
    pub avatar_url: Option<String>,
    pub bio: Option<String>
}
