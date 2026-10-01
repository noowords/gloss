use super::super::value_objects::{
    ServiceId,
    ServiceName,
    ServiceDescription,
    ServicePreviewUrl,
    ServiceCategory,
    ServiceKind,
    ServiceDurationMinutes,
    ServiceStatus
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Service {
    id: ServiceId,
    name: ServiceName,
    description: Option<ServiceDescription>,
    preview_url: Option<ServicePreviewUrl>,
    category: ServiceCategory,
    kind: ServiceKind,
    duration_minutes: ServiceDurationMinutes,
    status: ServiceStatus
}

// MARK: Constructors
impl Service {
    pub fn create(
        name: ServiceName,
        description: Option<ServiceDescription>,
        preview_url: Option<ServicePreviewUrl>,
        category: ServiceCategory,
        kind: ServiceKind,
        duration_minutes: ServiceDurationMinutes
    ) -> Self {
        let id = ServiceId::generate();
        let status = ServiceStatus::Inactive;
        
        Self { id, name, description, preview_url, category, kind, duration_minutes, status }
    }

    pub fn restore(
        id: ServiceId,
        name: ServiceName,
        description: Option<ServiceDescription>,
        preview_url: Option<ServicePreviewUrl>,
        category: ServiceCategory,
        kind: ServiceKind,
        duration_minutes: ServiceDurationMinutes,
        status: ServiceStatus
    ) -> Self {
        Self { id, name, description, preview_url, category, kind, duration_minutes, status }
    }
}

// MARK: Behavior
impl Service {
    pub fn change_name(&mut self, name: ServiceName) {
        self.name = name;
    }

    pub fn change_description(&mut self, description: Option<ServiceDescription>) {
        self.description = description;
    }

    pub fn change_preview_url(&mut self, preview_url: Option<ServicePreviewUrl>) {
        self.preview_url = preview_url;
    }

    pub fn change_category(&mut self, category: ServiceCategory) {
        self.category = category;
    }

    pub fn change_kind(&mut self, kind: ServiceKind) {
        self.kind = kind;
    }

    pub fn change_duration_minutes(&mut self, duration_minutes: ServiceDurationMinutes) {
        self.duration_minutes = duration_minutes;
    }

    pub fn activate(&mut self) {
        self.status = ServiceStatus::Active;
    }

    pub fn deactivate(&mut self) {
        self.status = ServiceStatus::Inactive;
    }
}

// MARK: Getters
impl Service {
    pub fn id(&self) -> ServiceId {
        self.id
    }

    pub fn name(&self) -> &ServiceName {
        &self.name
    }

    pub fn description(&self) -> Option<&ServiceDescription> {
        self.description.as_ref()
    }

    pub fn preview_url(&self) -> Option<&ServicePreviewUrl> {
        self.preview_url.as_ref()
    }

    pub fn category(&self) -> ServiceCategory {
        self.category
    }

    pub fn kind(&self) -> ServiceKind {
        self.kind
    }

    pub fn duration_minutes(&self) -> ServiceDurationMinutes {
        self.duration_minutes
    }

    pub fn status(&self) -> ServiceStatus {
        self.status
    }

    pub fn is_active(&self) -> bool {
        self.status == ServiceStatus::Active
    }
}
