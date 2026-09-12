use domain::aggregates::user::value_objects::{ UserId };

use crate::contracts::cqrs::query::{ Query };
use super::{ GetAccountQueryView };

#[derive(Clone)]
pub struct GetAccountQuery {
    pub id: UserId
}

impl Query for GetAccountQuery {
    type View = GetAccountQueryView;
    type Error = anyhow::Error;
}
