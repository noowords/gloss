#[derive(Debug, thiserror::Error)]
pub enum SalonScheduleExceptionError {
    // Value Objects
    #[error("salon schedule exception reason cannot be empty")]
    ReasonEmpty,
    
    #[error("salon schedule exception reason cannot exceed 255 characters")]
    ReasonTooLong
}
