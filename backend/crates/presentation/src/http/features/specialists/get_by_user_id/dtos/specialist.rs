use uuid::{ Uuid };
use serde::{ Serialize };

use application::features::specialists::queries::get_by_user_id::dtos::{ Specialist };

use super::{ HttpProfileDto };

#[derive(Serialize)]
pub struct HttpSpecialistDto {
    pub user_id: Uuid,
    pub profile: HttpProfileDto
}

impl From<Specialist> for HttpSpecialistDto {
    fn from(entity: Specialist) -> Self {
        Self {
            user_id: entity.user_id,
            profile: entity.profile.into()
        }
    }
}
