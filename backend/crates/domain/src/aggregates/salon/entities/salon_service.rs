use crate::aggregates::service::value_objects::ServiceId;

use super::super::value_objects::{
    SalonId,
    SalonServicePrice,
    SalonServiceStatus
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SalonService {
    salon_id: SalonId,
    service_id: ServiceId,
    price: SalonServicePrice,
    status: SalonServiceStatus
}

// MARK: Constructors
impl SalonService {
    pub fn create(
        salon_id: SalonId,
        service_id: ServiceId,
        price: SalonServicePrice
    ) -> Self {
        let status = SalonServiceStatus::Inactive;

        Self { salon_id, service_id, price, status }
    }

    pub fn restore(
        salon_id: SalonId,
        service_id: ServiceId,
        price: SalonServicePrice,
        status: SalonServiceStatus
    ) -> Self {
        Self { salon_id, service_id, price, status }
    }
}

// MARK: Behavior
impl SalonService {
    pub fn change_price(&mut self, price: SalonServicePrice) {
        self.price = price;
    }

    pub fn activate(&mut self) {
        self.status = SalonServiceStatus::Active;
    }

    pub fn deactivate(&mut self) {
        self.status = SalonServiceStatus::Inactive;
    }
}

// MARK: Getters
impl SalonService {
    pub fn salon_id(&self) -> SalonId {
        self.salon_id
    }

    pub fn service_id(&self) -> ServiceId {
        self.service_id
    }

    pub fn price(&self) -> &SalonServicePrice {
        &self.price
    }

    pub fn status(&self) -> SalonServiceStatus {
        self.status
    }

    pub fn is_active(&self) -> bool {
        self.status == SalonServiceStatus::Active
    }
}
