use serde::{ Deserialize };
use uuid::{ Uuid };

use application::projections::get_user_profile_by_id::{ GetUserProfileByIdQuery };

#[derive(Deserialize)]
pub struct GetUserProfileByIdRequest {
    pub id: Uuid
}

impl From<GetUserProfileByIdRequest> for GetUserProfileByIdQuery {
    fn from(req: GetUserProfileByIdRequest) -> Self {
        Self { id: req.id.into() }
    }
}
