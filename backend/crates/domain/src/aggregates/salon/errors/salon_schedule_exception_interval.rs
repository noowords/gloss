#[derive(Debug, thiserror::Error)]
pub enum SalonScheduleExceptionIntervalError {
    // Entity
    #[error("salon schedule exception interval start time must be earlier than end time")]
    InvalidInterval
}
