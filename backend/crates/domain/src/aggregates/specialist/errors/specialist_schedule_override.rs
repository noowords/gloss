#[derive(Debug, thiserror::Error)]
pub enum SpecialistScheduleOverrideError {
    // MARK: Value Objects
    #[error("specialist schedule override reason cannot be empty")]
    ReasonEmpty,
    
    #[error("specialist schedule override reason cannot exceed 255 characters")]
    ReasonTooLong
}
