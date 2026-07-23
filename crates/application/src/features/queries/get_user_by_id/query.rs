use domain::aggregates::user::value_objects::{ UserId };

use crate::interfaces::query::{ Query };
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
