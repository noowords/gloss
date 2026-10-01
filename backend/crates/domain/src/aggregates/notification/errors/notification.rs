#[derive(Debug, thiserror::Error)]
pub enum NotificationError {
    // Entity
    #[error("Notification is already read")]
    AlreadyRead,
    
    #[error("Notification read time cannot be before creation time")]
    InvalidReadTime,
    
    // Value Objects
    #[error("notification title cannot be empty")]
    TitleEmpty,
    
    #[error("notification title cannot exceed 128 characters")]
    TitleTooLong,

    #[error("notification message cannot be empty")]
    MessageEmpty,
    
    #[error("notification message cannot exceed 1024 characters")]
    MessageTooLong
}
