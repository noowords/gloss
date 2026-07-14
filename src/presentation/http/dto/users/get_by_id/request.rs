use serde::{ Deserialize };
use uuid::{ Uuid };

use crate::application::queries::users::get_by_id::{ GetUserByIdQuery };

#[derive(Deserialize)]
pub struct GetUserByIdRequest {
    pub id: Uuid
}

impl From<GetUserByIdRequest> for GetUserByIdQuery {
    fn from(req: GetUserByIdRequest) -> Self {
        Self { id: req.id }
    }
}
