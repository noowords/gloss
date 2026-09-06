use crate::aggregates::{
    service::value_objects::{ ServiceId },
    appointment::value_objects::{ AppointmentId }
};

use super::value_objects::{ AppointmentServiceLockedPrice };

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AppointmentService {
    appointment_id: AppointmentId,
    service_id: ServiceId,
    locked_price: AppointmentServiceLockedPrice
}

impl AppointmentService {
    pub fn create(
        appointment_id: AppointmentId,
        service_id: ServiceId,
        locked_price: AppointmentServiceLockedPrice
    ) -> Self {
        Self {
            appointment_id,
            service_id,
            locked_price
        }
    }
    
    pub fn restore(
        appointment_id: AppointmentId,
        service_id: ServiceId,
        locked_price: AppointmentServiceLockedPrice
    ) -> Self {
        Self {
            appointment_id,
            service_id,
            locked_price
        }
    }

    pub fn appointment_id(&self) -> AppointmentId {
        self.appointment_id
    }

    pub fn service_id(&self) -> ServiceId {
        self.service_id
    }

    pub fn locked_price(&self) -> AppointmentServiceLockedPrice {
        self.locked_price.clone()
    }
}
