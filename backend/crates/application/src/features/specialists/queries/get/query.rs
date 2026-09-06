use crate::contracts::cqrs::query::{ Query };
use super::{ GetSpecialistsQueryView };

#[derive(Clone)]
pub struct GetSpecialistsQuery;

impl Query for GetSpecialistsQuery {
    type View = GetSpecialistsQueryView;
    type Error = anyhow::Error;
}
