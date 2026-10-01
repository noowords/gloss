#[derive(Debug, thiserror::Error)]
pub enum AppointmentReviewError {
    // Value Objects
    #[error("appointment review rating must be between 1 and 5")]
    RatingOutOfRange,
    
    #[error("appointment review comment cannot be empty")]
    CommentEmpty,
    
    #[error("appointment review comment cannot exceed 1024 characters")]
    CommentTooLong
}
