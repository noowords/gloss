#[derive(Debug, thiserror::Error)]
pub enum OtpChallengeError {
    // Entity
    #[error("OTP challenge expiration time must be later than creation time")]
    InvalidExpirationTime,

    #[error("OTP challenge verification time must not be earlier than creation time")]
    InvalidVerificationTime,

    #[error("OTP challenge verification time must not be later than expiration time")]
    VerificationAfterExpiration,

    #[error("OTP challenge cannot be consumed before it is verified")]
    ConsumedWithoutVerification,

    #[error("OTP challenge consumption time must not be earlier than verification time")]
    InvalidConsumptionTime,

    #[error("OTP challenge is already verified")]
    AlreadyVerified,
    
    #[error("OTP challenge is already consumed")]
    AlreadyConsumed,
    
    // Value Objects
    #[error("OTP challenge provider cannot be empty")]
    ProviderEmpty,
    
    #[error("OTP challenge provider cannot exceed 32 characters")]
    ProviderTooLong,

    #[error("OTP challenge subject cannot be empty")]
    SubjectEmpty,
    
    #[error("OTP challenge subject cannot exceed 255 characters")]
    SubjectTooLong,

    #[error("OTP challenge code hash cannot be empty")]
    CodeHashEmpty
}
