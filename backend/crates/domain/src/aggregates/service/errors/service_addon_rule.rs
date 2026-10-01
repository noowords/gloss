#[derive(Debug, thiserror::Error)]
pub enum ServiceAddonRuleError {
    // Entity
    #[error("service addon rule primary service and addon service must be different")]
    SameService
}
