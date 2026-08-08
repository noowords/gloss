use serde::{ Serialize };

use application::features::specialists::queries::get_by_user_id::{ GetSpecialistByUserIdQueryView };

use super::dtos::{ HttpSpecialistDto };

#[derive(Serialize)]
pub struct GetSpecialistByUserIdResponse {
    pub data: HttpSpecialistDto
}

impl TryFrom<GetSpecialistByUserIdQueryView> for GetSpecialistByUserIdResponse {
    type Error = anyhow::Error;
    
    fn try_from(view: GetSpecialistByUserIdQueryView) -> Result<Self, Self::Error> {
        Ok(Self {
            data: view
                .ok_or_else(|| anyhow::anyhow!("Specialist not found"))?
                .into()
        })
    }
}
