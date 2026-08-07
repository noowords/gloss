use domain::aggregates::user::value_objects::{ UserId };

use crate::contracts::cqrs::query::{ Query };
use super::{ GetUserProfileByIdQueryView };

#[derive(Clone)]
pub struct GetUserProfileByIdQuery {
    pub id: UserId
}

impl Query for GetUserProfileByIdQuery {
    type View = GetUserProfileByIdQueryView;
    type Error = anyhow::Error;
}
