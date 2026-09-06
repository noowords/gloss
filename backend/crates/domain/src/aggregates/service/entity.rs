use super::value_objects::{ ServiceId, ServiceCategory, ServiceName, ServiceDescription, ServiceCoverUrl, ServicePrice, ServiceDuration, ServiceIsActive };

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Service {
    id: ServiceId,
    category: ServiceCategory,
    name: ServiceName,
    description: Option<ServiceDescription>,
    cover_url: Option<ServiceCoverUrl>,
    price: ServicePrice,
    duration: ServiceDuration,
    is_active: ServiceIsActive
}

impl Service {
    pub fn create(
        category: ServiceCategory,
        name: ServiceName,
        description: Option<ServiceDescription>,
        cover_url: Option<ServiceCoverUrl>,
        price: ServicePrice,
        duration: ServiceDuration,
        is_active: ServiceIsActive
    ) -> Self {
        Self {
            id: ServiceId::generate(),
            category,
            name,
            description,
            cover_url,
            price,
            duration,
            is_active
        }
    }
    
    pub fn restore(
        id: ServiceId,
        category: ServiceCategory,
        name: ServiceName,
        description: Option<ServiceDescription>,
        cover_url: Option<ServiceCoverUrl>,
        price: ServicePrice,
        duration: ServiceDuration,
        is_active: ServiceIsActive
    ) -> Self {
        Self {
            id,
            category,
            name,
            description,
            cover_url,
            price,
            duration,
            is_active
        }
    }

    pub fn id(&self) -> ServiceId {
        self.id
    }

    pub fn category(&self) -> ServiceCategory {
        self.category.clone()
    }

    pub fn name(&self) -> ServiceName {
        self.name.clone()
    }

    pub fn price(&self) -> ServicePrice {
        self.price.clone()
    }

    pub fn description(&self) -> Option<ServiceDescription> {
        self.description.clone()
    }

    pub fn cover_url(&self) -> Option<ServiceCoverUrl> {
        self.cover_url.clone()
    }

    pub fn duration(&self) -> ServiceDuration {
        self.duration
    }

    pub fn is_active(&self) -> ServiceIsActive {
        self.is_active
    }
}
