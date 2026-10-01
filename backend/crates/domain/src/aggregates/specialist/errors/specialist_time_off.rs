#[derive(Debug, thiserror::Error)]
pub enum SpecialistTimeOffError {
    // MARK: Entity
    #[error("specialist time off interval must have both start and end times")]
    IncompleteInterval,

    #[error("specialist time off start time must be earlier than end time")]
    InvalidInterval,
    
    // MARK: Value Objects
    #[error("specialist time off reason cannot be empty")]
    ReasonEmpty,
    
    #[error("specialist time off reason cannot exceed 255 characters")]
    ReasonTooLong
}
