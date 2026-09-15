use super::value_objects::{ ServiceId, ServiceName, ServiceDescription, ServicePreviewUrl, ServiceCategory, ServiceKind, ServiceDurationMinutes, ServiceIsActive };

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Service {
    id: ServiceId,
    name: ServiceName,
    description: Option<ServiceDescription>,
    preview_url: Option<ServicePreviewUrl>,
    category: ServiceCategory,
    kind: ServiceKind,
    duration_minutes: ServiceDurationMinutes,
    is_active: ServiceIsActive
}

impl Service {
    pub fn create(
        name: ServiceName,
        description: Option<ServiceDescription>,
        preview_url: Option<ServicePreviewUrl>,
        category: ServiceCategory,
        kind: ServiceKind,
        duration_minutes: ServiceDurationMinutes
    ) -> Result<Self, anyhow::Error> {
        let is_active = ServiceIsActive::try_from(true)?;
        let id = ServiceId::generate();
        Self::restore(
            id,
            name,
            description,
            preview_url,
            category,
            kind,
            duration_minutes,
            is_active
        )
    }

    pub fn restore(
        id: ServiceId,
        name: ServiceName,
        description: Option<ServiceDescription>,
        preview_url: Option<ServicePreviewUrl>,
        category: ServiceCategory,
        kind: ServiceKind,
        duration_minutes: ServiceDurationMinutes,
        is_active: ServiceIsActive
    ) -> Result<Self, anyhow::Error> {
        Ok(Self {
            id,
            name,
            description,
            preview_url,
            category,
            kind,
            duration_minutes,
            is_active
        })
    }

    pub fn id(&self) -> ServiceId {
        self.id
    }

    pub fn name(&self) -> ServiceName {
        self.name.clone()
    }

    pub fn description(&self) -> Option<ServiceDescription> {
        self.description.clone()
    }

    pub fn preview_url(&self) -> Option<ServicePreviewUrl> {
        self.preview_url.clone()
    }

    pub fn category(&self) -> ServiceCategory {
        self.category.clone()
    }

    pub fn kind(&self) -> ServiceKind {
        self.kind
    }

    pub fn duration_minutes(&self) -> ServiceDurationMinutes {
        self.duration_minutes
    }

    pub fn is_active(&self) -> ServiceIsActive {
        self.is_active
    }
}
