use domain::aggregates::user::value_objects::{ UserId };

use crate::contracts::cqrs::query::{ Query };
use super::{ GetAccountProfileQueryView };

#[derive(Clone)]
pub struct GetAccountProfileQuery {
    pub user_id: UserId
}

impl Query for GetAccountProfileQuery {
    type View = GetAccountProfileQueryView;
    type Error = anyhow::Error;
}
