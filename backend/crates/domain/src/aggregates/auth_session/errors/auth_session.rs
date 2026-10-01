#[derive(Debug, thiserror::Error)]
pub enum AuthSessionError {
    // Entity
    #[error("auth session expiration time must be later than creation time")]
    InvalidExpirationTime,

    #[error("auth session last used time must not be earlier than creation time")]
    InvalidLastUsedTime,

    #[error("auth session revoked time must not be earlier than creation time")]
    InvalidRevokedTime,

    #[error("auth session last used time must not be later than revoked time")]
    LastUsedAfterRevocation,

    // Value Objects
    #[error("auth session refresh token hash cannot be empty")]
    RefreshTokenHashEmpty,

    #[error("auth session user agent cannot be empty")]
    UserAgentEmpty,

    #[error("auth session user agent cannot exceed 512 characters")]
    UserAgentTooLong,

    #[error("Auth session is already revoked")]
    AlreadyRevoked
}
