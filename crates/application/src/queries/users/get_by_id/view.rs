use serde::{ Serialize };
use sqlx::{ FromRow };
use uuid::{ Uuid };

#[derive(Debug, Clone, FromRow, Serialize)]
pub struct Profile {
    pub first_name: String,
    pub last_name: String,
    pub avatar_url: Option<String>,
    pub bio: Option<String>
}

#[derive(Debug, Clone, FromRow, Serialize)]
pub struct GetUserByIdView {
    pub id: Uuid,
    pub role: String,
    pub phone: Option<String>,
    
    #[sqlx(flatten)]
    pub profile: Profile
}
