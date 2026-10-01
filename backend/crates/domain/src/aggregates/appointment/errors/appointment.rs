#[derive(Debug, thiserror::Error)]
pub enum AppointmentError {
    // Entity 
    #[error("Appointment duration does not match its time range")]
    InvalidDuration,
    
    #[error("appointment end time must be later than start time")]
    InvalidTimeRange,

    #[error("appointment start time must be later than creation time")]
    InvalidStartTime,

    #[error("appointment cancelled status must have cancellation time")]
    MissingCancellationTime,

    #[error("appointment non-cancelled status must not have cancellation data")]
    UnexpectedCancellationData,

    #[error("appointment cancellation time must not be earlier than creation time")]
    InvalidCancellationTime,

    #[error("Appointment has invalid status transition")]
    InvalidStatusTransition,
    
    // Value Objects
    #[error("appointment cancellation reason cannot be empty")]
    CancellationReasonEmpty,
    
    #[error("appointment cancellation reason cannot exceed 255 characters")]
    CancellationReasonTooLong,

    #[error("appointment total price snapshot cannot be negative")]
    TotalPriceSnapshotNegative,

    #[error("appointment total duration minutes snapshot cannot be zero")]
    TotalDurationMinutesSnapshotZero
}
