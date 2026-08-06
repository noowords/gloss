use crate::aggregates::user::value_objects::{ UserId };

use super::value_objects::{ ServiceId, ServiceName, ServicePrice, ServiceDuration, ServiceIsActive };

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Service {
    id: ServiceId,
    specialist_id: UserId,
    name: ServiceName,
    price: ServicePrice,
    duration: ServiceDuration,
    is_active: ServiceIsActive
}

impl Service {
    pub fn create(
        specialist_id: UserId,
        name: ServiceName,
        price: ServicePrice,
        duration: ServiceDuration,
        is_active: ServiceIsActive
    ) -> Self {
        Self {
            id: ServiceId::generate(),
            specialist_id,
            name,
            price,
            duration,
            is_active
        }
    }
    
    pub fn restore(
        id: ServiceId,
        specialist_id: UserId,
        name: ServiceName,
        price: ServicePrice,
        duration: ServiceDuration,
        is_active: ServiceIsActive
    ) -> Self {
        Self {
            id,
            specialist_id,
            name,
            price,
            duration,
            is_active
        }
    }

    pub fn id(&self) -> ServiceId {
        self.id
    }

    pub fn specialist_id(&self) -> UserId {
        self.specialist_id
    }

    pub fn name(&self) -> ServiceName {
        self.name.clone()
    }

    pub fn price(&self) -> ServicePrice {
        self.price.clone()
    }

    pub fn duration(&self) -> ServiceDuration {
        self.duration
    }

    pub fn is_active(&self) -> ServiceIsActive {
        self.is_active
    }
}
