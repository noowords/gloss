#[derive(Debug, thiserror::Error)]
pub enum AppointmentServiceError {
    // Value Objects
    #[error("appointment service name snapshot cannot be empty")]
    NameSnapshotEmpty,

    #[error("appointment service name snapshot cannot exceed 128 characters")]
    NameSnapshotTooLong,

    #[error("appointment service price snapshot cannot be negative")]
    PriceSnapshotNegative,

    #[error("appointment service duration minutes snapshot cannot be zero")]
    DurationMinutesSnapshotZero
}
