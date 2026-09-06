use crate::contracts::cqrs::query::{ Query };
use super::{ GetAppointmentsQueryView };

#[derive(Clone)]
pub struct GetAppointmentsQuery;

impl Query for GetAppointmentsQuery {
    type View = GetAppointmentsQueryView;
    type Error = anyhow::Error;
}
