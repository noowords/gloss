use serde::{ Deserialize };
use uuid::{ Uuid };

use application::features::specialists::queries::get_services_by_user_id::{ GetSpecialistServicesByUserIdQuery };

#[derive(Deserialize)]
pub struct GetSpecialistProfileByUserIdRequest {
    pub id: Uuid
}

impl From<GetSpecialistProfileByUserIdRequest> for GetSpecialistServicesByUserIdQuery {
    fn from(req: GetSpecialistProfileByUserIdRequest) -> Self {
        Self { user_id: req.id.into() }
    }
}
