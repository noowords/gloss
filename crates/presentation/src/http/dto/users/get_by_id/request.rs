use serde::{ Deserialize };
use uuid::{ Uuid };

use application::projections::queries::{ GetUserByIdQuery };

#[derive(Deserialize)]
pub struct GetUserByIdRequest {
    pub id: Uuid
}

impl From<GetUserByIdRequest> for GetUserByIdQuery {
    fn from(req: GetUserByIdRequest) -> Self {
        Self { id: req.id.into() }
    }
}
