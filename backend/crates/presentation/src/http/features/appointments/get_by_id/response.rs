use serde::{ Serialize };

use application::features::appointments::queries::get_by_id::{ GetAppointmentByIdQueryView };

use super::dtos::{ HttpAppointmentDto };

#[derive(Serialize)]
pub struct GetAppointmentByIdResponse {
    pub data: HttpAppointmentDto
}

impl TryFrom<GetAppointmentByIdQueryView> for GetAppointmentByIdResponse {
    type Error = anyhow::Error;
    
    fn try_from(view: GetAppointmentByIdQueryView) -> Result<Self, Self::Error> {
        Ok(Self {
            data: view
                .ok_or_else(|| anyhow::anyhow!("Appointment not found"))?
                .into()
        })
    }
}
