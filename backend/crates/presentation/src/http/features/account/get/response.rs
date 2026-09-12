use serde::{ Serialize };

use application::features::account::queries::get::{ GetAccountQueryView };

use super::dtos::{ HttpUserDto };

#[derive(Serialize)]
pub struct GetAccountResponse {
    pub data: HttpUserDto
}

impl TryFrom<GetAccountQueryView> for GetAccountResponse {
    type Error = anyhow::Error;
    
    fn try_from(view: GetAccountQueryView) -> Result<Self, Self::Error> {
        Ok(Self {
            data: view
                .ok_or_else(|| anyhow::anyhow!("User not found"))?
                .into()
        })
    }
}
