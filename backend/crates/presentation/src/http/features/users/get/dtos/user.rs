use uuid::{ Uuid };
use serde::{ Serialize };

use application::features::users::queries::get::dtos::{ User };

use super::{ HttpProfileDto };

#[derive(Serialize)]
pub struct HttpUserDto {
    pub id: Uuid,
    pub role: String,
    pub profile: HttpProfileDto
}

impl From<User> for HttpUserDto {
    fn from(entity: User) -> Self {
        Self {
            id: entity.id,
            role: entity.role.clone(),
            profile: entity.profile.into()
        }
    }
}
