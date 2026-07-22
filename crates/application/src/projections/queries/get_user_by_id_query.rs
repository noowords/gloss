use domain::user::value_objects::{ UserId };

use crate::buses::query_bus::{ Query };

use crate::projections::{
    query_results::{ GetUserByIdQueryResult },
    query_handlers::{ GetUserByIdQueryHandler }
};

#[derive(Clone)]
pub struct GetUserByIdQuery {
    pub id: UserId
}

impl Query for GetUserByIdQuery {
    type Result = GetUserByIdQueryResult;
    type Error = anyhow::Error;
    
    type Handler = GetUserByIdQueryHandler;
}
