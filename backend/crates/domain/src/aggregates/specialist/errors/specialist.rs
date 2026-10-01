#[derive(Debug, thiserror::Error)]
pub enum SpecialistError {
    // MARK: Value Objects
    #[error("specialist bio cannot be empty")]
    BioEmpty,

    #[error("specialist bio cannot exceed 1024 characters")]
    BioTooLong
}
