#[derive(Debug, thiserror::Error)]
pub enum ServiceError {
    // MARK: Value Objects
    #[error("service name cannot be empty")]
    NameEmpty,
    
    #[error("service name cannot exceed 128 characters")]
    NameTooLong,
    
    #[error("service description cannot be empty")]
    DescriptionEmpty,
    
    #[error("service description cannot exceed 1024 characters")]
    DescriptionTooLong,

    #[error("service preview URL cannot be empty")]
    PreviewUrlEmpty,
    
    #[error("service preview URL cannot exceed 2048 characters")]
    PreviewUrlTooLong,

    #[error("service duration must be greater than zero")]
    DurationMinutesZero
}
