use serde::{ Deserialize };
use uuid::{ Uuid };

use application::features::appointments::queries::get_by_id::{ GetAppointmentByIdQuery };

#[derive(Deserialize)]
pub struct GetAppointmentByIdRequest {
    pub id: Uuid
}

impl From<GetAppointmentByIdRequest> for GetAppointmentByIdQuery {
    fn from(req: GetAppointmentByIdRequest) -> Self {
        Self { id: req.id.into() }
    }
}
