use serde::{ Serialize };

use application::features::users::queries::get_by_id::{ GetUserByIdQueryView };

use super::dtos::{ HttpUserDto };

#[derive(Serialize)]
pub struct GetUserByIdResponse {
    pub data: HttpUserDto
}

impl TryFrom<GetUserByIdQueryView> for GetUserByIdResponse {
    type Error = anyhow::Error;
    
    fn try_from(view: GetUserByIdQueryView) -> Result<Self, Self::Error> {
        Ok(Self {
            data: view
                .ok_or_else(|| anyhow::anyhow!("User not found"))?
                .into()
        })
    }
}
