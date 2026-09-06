use serde::{ Serialize };

use application::features::specialists::queries::get::dtos::{ Profile };

#[derive(Serialize)]
pub struct HttpProfileDto {
    pub first_name: String,
    pub last_name: Option<String>,
    pub avatar_url: Option<String>,
    pub bio: Option<String>
}

impl From<Profile> for HttpProfileDto {
    fn from(entity: Profile) -> Self {
        Self {
            first_name: entity.first_name,
            last_name: entity.last_name,
            avatar_url: entity.avatar_url,
            bio: entity.bio
        }
    }
}
