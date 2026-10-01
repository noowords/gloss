#[derive(Debug, thiserror::Error)]
pub enum SpecialistScheduleIntervalError {
    // Entity
    #[error("specialist schedule interval start time must be earlier than end time")]
    InvalidInterval
}
