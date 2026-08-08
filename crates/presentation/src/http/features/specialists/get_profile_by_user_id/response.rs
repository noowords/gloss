use serde::{ Serialize };

use application::features::specialists::queries::get_services_by_user_id::{ GetSpecialistServicesByUserIdQueryView };

use super::dtos::{ HttpServiceDto };

#[derive(Serialize)]
pub struct GetSpecialistProfileByUserIdResponse {
    pub data: Vec<HttpServiceDto>
}

impl TryFrom<GetSpecialistServicesByUserIdQueryView> for GetSpecialistProfileByUserIdResponse {
    type Error = anyhow::Error;
    
    fn try_from(view: GetSpecialistServicesByUserIdQueryView) -> Result<Self, Self::Error> {
        Ok(Self {
            data: view
                .into_iter()
                .map(|s| s.into())
                .collect()
        })
    }
}
