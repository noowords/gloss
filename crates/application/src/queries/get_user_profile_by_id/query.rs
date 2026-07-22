use domain::user::value_objects::{ UserId };

use crate::buses::query_bus::{ Query };
use super::{ GetUserProfileByIdQueryResult, GetUserProfileByIdQueryHandler };

#[derive(Clone)]
pub struct GetUserProfileByIdQuery {
    pub id: UserId
}

impl Query for GetUserProfileByIdQuery {
    type Result = GetUserProfileByIdQueryResult;
    type Error = anyhow::Error;
    
    type Handler = GetUserProfileByIdQueryHandler;
}

