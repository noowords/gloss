use serde::{ Serialize };

use application::features::appointments::queries::get::{ GetAppointmentsQueryView };

use super::dtos::{ HttpAppointmentDto };

#[derive(Serialize)]
pub struct GetAppointmentsResponse {
    pub data: Vec<HttpAppointmentDto>
}

impl From<GetAppointmentsQueryView> for GetAppointmentsResponse {
    fn from(view: GetAppointmentsQueryView) -> Self {
        Self {
            data: view
                .into_iter()
                .map(|appointment| appointment.into())
                .collect()
        }
    }
}
