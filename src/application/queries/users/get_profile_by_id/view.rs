use serde::{ Serialize };
use sqlx::{ FromRow };

#[derive(Debug, Clone, FromRow, Serialize)]
pub struct GetUserProfileByIdView {
    pub first_name: String,
    pub last_name: String,
    pub avatar_url: Option<String>,
    pub bio: Option<String>
}
