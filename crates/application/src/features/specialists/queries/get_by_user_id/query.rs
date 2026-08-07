use domain::aggregates::user::value_objects::{ UserId };

use crate::contracts::cqrs::query::{ Query };
use super::{ GetSpecialistByUserIdQueryView };

#[derive(Clone)]
pub struct GetSpecialistByUserIdQuery {
    pub user_id: UserId
}

impl Query for GetSpecialistByUserIdQuery {
    type View = GetSpecialistByUserIdQueryView;
    type Error = anyhow::Error;
}
