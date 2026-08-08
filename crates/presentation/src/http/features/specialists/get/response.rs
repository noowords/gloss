use serde::{ Serialize };

use application::features::specialists::queries::get::{ GetSpecialistsQueryView };

use super::dtos::{ HttpSpecialistDto };

#[derive(Serialize)]
pub struct GetSpecialistsResponse {
    pub data: Vec<HttpSpecialistDto>
}

impl From<GetSpecialistsQueryView> for GetSpecialistsResponse {
    fn from(view: GetSpecialistsQueryView) -> Self {
        Self {
            data: view
                .into_iter()
                .map(|s| s.into())
                .collect()
        }
    }
}
