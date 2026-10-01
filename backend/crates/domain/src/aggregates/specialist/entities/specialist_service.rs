use crate::aggregates::{
    salon::value_objects::SalonId,
    service::value_objects::ServiceId
};

use super::super::value_objects::{
    SpecialistId
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SpecialistService {
    specialist_id: SpecialistId,
    salon_id: SalonId,
    service_id: ServiceId
}

// MARK: Constructors
impl SpecialistService {
    pub fn create(
        specialist_id: SpecialistId,
        salon_id: SalonId,
        service_id: ServiceId
    ) -> Self {
        Self { specialist_id,  salon_id, service_id }
    }

    pub fn restore(
        specialist_id: SpecialistId,
        salon_id: SalonId,
        service_id: ServiceId
    ) -> Self {
        Self { specialist_id,  salon_id, service_id }
    }
}

// MARK: Getters
impl SpecialistService {
    pub fn specialist_id(&self) -> SpecialistId {
        self.specialist_id
    }

    pub fn salon_id(&self) -> SalonId {
        self.salon_id
    }

    pub fn service_id(&self) -> ServiceId {
        self.service_id
    }
}
