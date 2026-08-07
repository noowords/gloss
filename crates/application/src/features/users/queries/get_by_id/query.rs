use domain::aggregates::user::value_objects::{ UserId };

use crate::contracts::cqrs::query::{ Query };
use super::{ GetUserByIdQueryView, GetUserByIdQueryHandler };

#[derive(Clone)]
pub struct GetUserByIdQuery {
    pub id: UserId
}

impl Query for GetUserByIdQuery {
    type View = GetUserByIdQueryView;
    type Error = anyhow::Error;

    type Handler = GetUserByIdQueryHandler;
}
