use crate::aggregates::{
    user::value_objects::{ UserId },
    service::value_objects::{ ServiceId }
};

use super::value_objects::{ SpecialistServiceIsActive };

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SpecialistService {
    specialist_id: UserId,
    service_id: ServiceId,
    is_active: SpecialistServiceIsActive
}

impl SpecialistService {
    pub fn create(
        specialist_id: UserId,
        service_id: ServiceId,
        is_active: SpecialistServiceIsActive
    ) -> Self {
        Self {
            specialist_id,
            service_id,
            is_active
        }
    }
    
    pub fn restore(
        specialist_id: UserId,
        service_id: ServiceId,
        is_active: SpecialistServiceIsActive
    ) -> Self {
        Self {
            specialist_id,
            service_id,
            is_active
        }
    }

    pub fn specialist_id(&self) -> UserId {
        self.specialist_id
    }

    pub fn service_id(&self) -> ServiceId {
        self.service_id
    }

    pub fn is_active(&self) -> SpecialistServiceIsActive {
        self.is_active
    }
}
