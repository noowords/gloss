use serde::{ Serialize };

use application::features::users::queries::get_profile_by_id::{ GetUserProfileByIdQueryView };

use super::dtos::{ HttpProfileDto };

#[derive(Serialize)]
pub struct GetUserProfileByIdResponse {
    pub data: HttpProfileDto
}

impl TryFrom<GetUserProfileByIdQueryView> for GetUserProfileByIdResponse {
    type Error = anyhow::Error;
    
    fn try_from(view: GetUserProfileByIdQueryView) -> Result<Self, Self::Error> {
        Ok(Self {
            data: view
                .ok_or_else(|| anyhow::anyhow!("User profile not found"))?
                .into()
        })
    }
}
