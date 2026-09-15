use crate::aggregates::services::service::value_objects::{ ServiceId };

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ServiceAddonRule {
    primary_service_id: ServiceId,
    addon_service_id: ServiceId
}

impl ServiceAddonRule {
    pub fn create(
        primary: &crate::aggregates::services::service::Service,
        addon: &crate::aggregates::services::service::Service
    ) -> Result<Self, anyhow::Error> {
        use crate::aggregates::services::service::value_objects::{ ServiceKind };

        if primary.kind() != ServiceKind::Primary || addon.kind() != ServiceKind::Addon {
            anyhow::bail!("An addon rule requires a primary service and an addon service");
        }

        Self::restore(primary.id(), addon.id())
    }

    pub fn restore(
        primary_service_id: ServiceId,
        addon_service_id: ServiceId
    ) -> Result<Self, anyhow::Error> {
        if primary_service_id == addon_service_id {
            anyhow::bail!("A service cannot be its own addon");
        }

        Ok(Self {
            primary_service_id,
            addon_service_id
        })
    }

    pub fn primary_service_id(&self) -> ServiceId {
        self.primary_service_id
    }

    pub fn addon_service_id(&self) -> ServiceId {
        self.addon_service_id
    }
}
