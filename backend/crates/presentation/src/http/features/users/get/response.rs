use serde::{ Serialize };

use application::features::users::queries::get::{ GetUsersQueryView };

use super::dtos::{ HttpUserDto };

#[derive(Serialize)]
pub struct GetUsersResponse {
    pub data: Vec<HttpUserDto>
}

impl From<GetUsersQueryView> for GetUsersResponse {
    fn from(view: GetUsersQueryView) -> Self {
        Self {
            data: view
                .into_iter()
                .map(|user| user.into())
                .collect()
        }
    }
}
