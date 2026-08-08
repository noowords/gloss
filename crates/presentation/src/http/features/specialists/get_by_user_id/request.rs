use serde::{ Deserialize };
use uuid::{ Uuid };

use application::features::specialists::queries::get_by_user_id::{ GetSpecialistByUserIdQuery };

#[derive(Deserialize)]
pub struct GetSpecialistByUserIdRequest {
    pub user_id: Uuid
}

impl From<GetSpecialistByUserIdRequest> for GetSpecialistByUserIdQuery {
    fn from(req: GetSpecialistByUserIdRequest) -> Self {
        Self { user_id: req.user_id.into() }
    }
}
