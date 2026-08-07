use crate::contracts::cqrs::query::{ Query };
use super::{ GetUsersQueryView, GetUsersQueryHandler };

#[derive(Clone)]
pub struct GetUsersQuery;

impl Query for GetUsersQuery {
    type View = GetUsersQueryView;
    type Error = anyhow::Error;

    type Handler = GetUsersQueryHandler;
}
