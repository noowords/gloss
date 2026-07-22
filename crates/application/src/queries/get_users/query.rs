use crate::buses::query_bus::{ Query };
use super::{ GetUsersQueryResult, GetUsersQueryHandler };

#[derive(Clone)]
pub struct GetUsersQuery;

impl Query for GetUsersQuery {
    type Result = GetUsersQueryResult;
    type Error = anyhow::Error;

    type Handler = GetUsersQueryHandler;
}

