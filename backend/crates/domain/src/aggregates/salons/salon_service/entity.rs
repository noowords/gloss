use crate::aggregates::salons::salon::value_objects::{ SalonId };
use crate::aggregates::services::service::value_objects::{ ServiceId };

use super::value_objects::{ SalonServicePrice, SalonServiceIsActive };

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SalonService {
    salon_id: SalonId,
    service_id: ServiceId,
    price: SalonServicePrice,
    is_active: SalonServiceIsActive
}

impl SalonService {
    pub fn create(
        salon_id: SalonId,
        service_id: ServiceId,
        price: SalonServicePrice
    ) -> Result<Self, anyhow::Error> {
        let is_active = SalonServiceIsActive::try_from(true)?;
        Self::restore(
            salon_id,
            service_id,
            price,
            is_active
        )
    }

    pub fn restore(
        salon_id: SalonId,
        service_id: ServiceId,
        price: SalonServicePrice,
        is_active: SalonServiceIsActive
    ) -> Result<Self, anyhow::Error> {
        Ok(Self {
            salon_id,
            service_id,
            price,
            is_active
        })
    }

    pub fn salon_id(&self) -> SalonId {
        self.salon_id
    }

    pub fn service_id(&self) -> ServiceId {
        self.service_id
    }

    pub fn price(&self) -> SalonServicePrice {
        self.price.clone()
    }

    pub fn is_active(&self) -> SalonServiceIsActive {
        self.is_active
    }
}
