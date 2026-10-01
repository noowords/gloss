#[derive(Debug, thiserror::Error)]
pub enum SpecialistScheduleError {
    // Entity
    #[error("specialist schedule effective until date must not be earlier than effective from date")]
    InvalidEffectivePeriod,

    #[error("Specialist schedule is already closed")]
    AlreadyClosed
}
