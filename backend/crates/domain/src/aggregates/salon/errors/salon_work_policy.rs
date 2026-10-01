#[derive(Debug, thiserror::Error)]
pub enum SalonWorkPolicyError {
    // MARK: Value Objects
    #[error("salon work policy booking horizon must be greater than zero")]
    BookingHorizonDaysZero,

    #[error("salon work policy booking step must be greater than zero")]
    BookingStepMinutesZero,

    #[error("salon work policy weekly work minutes must be greater than zero")]
    WeeklyWorkMinutesZero
}
