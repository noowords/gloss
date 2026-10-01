#[derive(Debug, thiserror::Error)]
pub enum SalonScheduleIntervalError {
    // Entity
    #[error("salon schedule interval start time must be earlier than end time")]
    InvalidInterval
}
