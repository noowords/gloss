use serde::{ Deserialize };
use uuid::{ Uuid };

use application::features::specialists::queries::get_services_by_user_id::{ GetSpecialistServicesByUserIdQuery };

#[derive(Deserialize)]
pub struct GetSpecialistServicesByUserIdRequest {
    pub id: Uuid
}

impl From<GetSpecialistServicesByUserIdRequest> for GetSpecialistServicesByUserIdQuery {
    fn from(req: GetSpecialistServicesByUserIdRequest) -> Self {
        Self { user_id: req.id.into() }
    }
}
