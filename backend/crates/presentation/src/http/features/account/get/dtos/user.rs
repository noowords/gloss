use uuid::{ Uuid };
use serde::{ Serialize };

use application::features::account::queries::get::dtos::{ User };

#[derive(Serialize)]
pub struct HttpUserDto {
    pub id: Uuid,
    pub role: String
}

impl From<User> for HttpUserDto {
    fn from(entity: User) -> Self {
        Self {
            id: entity.id,
            role: entity.role.clone()
        }
    }
}
