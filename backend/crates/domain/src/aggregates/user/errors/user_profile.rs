#[derive(Debug, thiserror::Error)]
pub enum UserProfileError {
    // Value Objects
    #[error("user profile first name cannot be empty")]
    FirstNameEmpty,
    
    #[error("user profile first name cannot exceed 128 characters")]
    FirstNameTooLong,

    #[error("user profile last name cannot be empty")]
    LastNameEmpty,
    
    #[error("user profile last name cannot exceed 128 characters")]
    LastNameTooLong,

    #[error("user profile avatar URL cannot be empty")]
    AvatarUrlEmpty,
    
    #[error("user profile avatar URL cannot exceed 2048 characters")]
    AvatarUrlTooLong
}
