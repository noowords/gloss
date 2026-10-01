#[derive(Debug, thiserror::Error)]
pub enum SpecialistScheduleOverrideIntervalError {
    // Entity
    #[error("specialist schedule override interval start time must be earlier than end time")]
    InvalidInterval
}
