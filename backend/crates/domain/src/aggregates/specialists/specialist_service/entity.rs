use crate::aggregates::specialists::specialist::value_objects::{ SpecialistId };
use crate::aggregates::salons::salon::value_objects::{ SalonId };
use crate::aggregates::services::service::value_objects::{ ServiceId };

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SpecialistService {
    specialist_id: SpecialistId,
    salon_id: SalonId,
    service_id: ServiceId
}

impl SpecialistService {
    pub fn create(
        specialist_id: SpecialistId,
        salon_id: SalonId,
        service_id: ServiceId
    ) -> Result<Self, anyhow::Error> {
        Self::restore(
            specialist_id,
            salon_id,
            service_id
        )
    }

    pub fn restore(
        specialist_id: SpecialistId,
        salon_id: SalonId,
        service_id: ServiceId
    ) -> Result<Self, anyhow::Error> {
        Ok(Self {
            specialist_id,
            salon_id,
            service_id
        })
    }

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
