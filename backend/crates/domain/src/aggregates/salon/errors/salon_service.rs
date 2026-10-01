#[derive(Debug, thiserror::Error)]
pub enum SalonServiceError {
    // MARK: Value Objects
    #[error("salon service price cannot be negative")]
    PriceNegative
}
