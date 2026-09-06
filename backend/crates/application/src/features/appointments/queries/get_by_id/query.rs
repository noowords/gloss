use domain::aggregates::appointment::value_objects::{ AppointmentId };

use crate::contracts::cqrs::query::{ Query };
use super::{ GetAppointmentByIdQueryView };

#[derive(Clone)]
pub struct GetAppointmentByIdQuery {
    pub id: AppointmentId
}

impl Query for GetAppointmentByIdQuery {
    type View = GetAppointmentByIdQueryView;
    type Error = anyhow::Error;
}
