#[derive(Debug, thiserror::Error)]
pub enum NotificationDeliveryError {
    // Entity
    #[error("notification delivery pending status must not have attempted or delivered time")]
    InvalidPendingState,

    #[error("notification delivery processing status must have attempted time and must not have delivered time")]
    InvalidProcessingState,

    #[error("notification delivery failed status must have attempted time and must not have delivered time")]
    InvalidFailedState,

    #[error("notification delivery delivered status must have attempted and delivered times")]
    InvalidDeliveredState,

    #[error("notification delivery delivered time must not be earlier than attempted time")]
    InvalidDeliveryTime,

    #[error("Notification delivery has invalid status transition")]
    InvalidStatusTransition
}
