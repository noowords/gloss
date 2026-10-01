#[derive(Debug, thiserror::Error)]
pub enum UserProviderError {
    // MARK: Entity
    #[error("user provider is already verified")]
    AlreadyVerified,
    
    // MARK: Value Objects
    #[error("user provider subject cannot be empty")]
    SubjectEmpty,
    
    #[error("user provider subject cannot exceed 255 characters")]
    SubjectTooLong
}
