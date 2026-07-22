use domain::user::value_objects::{ UserId };

use crate::{ Query };
use super::{ GetUserByIdQueryResult, GetUserByIdQueryHandler };

#[derive(Clone)]
pub struct GetUserByIdQuery {
    pub id: UserId
}

impl Query for GetUserByIdQuery {
    type Result = GetUserByIdQueryResult;
    type Error = anyhow::Error;

    type Handler = GetUserByIdQueryHandler;
}
