use serde::{ Serialize };

use application::features::account::queries::get_profile::{ GetAccountProfileQueryView };

use super::dtos::{ HttpProfileDto };

#[derive(Serialize)]
pub struct GetAccountProfileResponse {
    pub data: HttpProfileDto
}

impl TryFrom<GetAccountProfileQueryView> for GetAccountProfileResponse {
    type Error = anyhow::Error;
    
    fn try_from(view: GetAccountProfileQueryView) -> Result<Self, Self::Error> {
        Ok(Self {
            data: view
                .ok_or_else(|| anyhow::anyhow!("User profile not found"))?
                .into()
        })
    }
}
