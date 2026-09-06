use uuid::{ Uuid };

use super::{ Profile };

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct User {
    pub id: Uuid,
    pub role: String,
    pub profile: Profile
}
