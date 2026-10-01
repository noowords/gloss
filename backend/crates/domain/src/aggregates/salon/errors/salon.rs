#[derive(Debug, thiserror::Error)]
pub enum SalonError {
    // Value Objects
    #[error("salon city cannot be empty")]
    CityEmpty,
    
    #[error("salon city cannot exceed 128 characters")]
    CityTooLong,
    
    #[error("salon address cannot be empty")]
    AddressEmpty,
    
    #[error("salon address cannot exceed 255 characters")]
    AddressTooLong,

    #[error("salon name cannot be empty")]
    NameEmpty,
    
    #[error("salon name cannot exceed 128 characters")]
    NameTooLong,

    #[error("salon code cannot be empty")]
    CodeEmpty,
    
    #[error("salon code cannot exceed 32 characters")]
    CodeTooLong,
    
    #[error("salon code must contain only uppercase ASCII letters and digits")]
    CodeInvalidFormat
}
