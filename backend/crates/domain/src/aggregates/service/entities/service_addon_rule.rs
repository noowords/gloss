use super::super::{ errors::ServiceAddonRuleError, value_objects::ServiceId };

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ServiceAddonRule {
    primary_service_id: ServiceId,
    addon_service_id: ServiceId
}

// MARK: Constructors
impl ServiceAddonRule {
    pub fn create(
        primary_service_id: ServiceId,
        addon_service_id: ServiceId
    ) -> Result<Self, ServiceAddonRuleError> {
        Self::validate_services(primary_service_id, addon_service_id)?;
        
        Ok(Self { primary_service_id, addon_service_id })
    }

    pub fn restore(
        primary_service_id: ServiceId,
        addon_service_id: ServiceId
    ) -> Result<Self, ServiceAddonRuleError> {
        Self::validate_services(primary_service_id, addon_service_id)?;
        
        Ok(Self { primary_service_id, addon_service_id })
    }
}

// MARK: Validation
impl ServiceAddonRule {
    fn validate_services(
        primary_service_id: ServiceId,
        addon_service_id: ServiceId,
    ) -> Result<(), ServiceAddonRuleError> {
        if primary_service_id == addon_service_id {
            return Err(ServiceAddonRuleError::SameService);
        }

        Ok(())
    }
}

// MARK: Getters
impl ServiceAddonRule {
    pub fn primary_service_id(&self) -> ServiceId {
        self.primary_service_id
    }

    pub fn addon_service_id(&self) -> ServiceId {
        self.addon_service_id
    }
}
