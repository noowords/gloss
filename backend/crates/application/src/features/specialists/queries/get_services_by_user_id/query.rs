use domain::aggregates::user::value_objects::{ UserId };

use crate::contracts::cqrs::query::{ Query };
use super::{ GetSpecialistServicesByUserIdQueryView };

#[derive(Clone)]
pub struct GetSpecialistServicesByUserIdQuery {
    pub user_id: UserId
}

impl Query for GetSpecialistServicesByUserIdQuery {
    type View = GetSpecialistServicesByUserIdQueryView;
    type Error = anyhow::Error;
}
