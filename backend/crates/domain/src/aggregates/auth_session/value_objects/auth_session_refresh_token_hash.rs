use super::super::errors::AuthSessionError;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AuthSessionRefreshTokenHash(Vec<u8>);

// MARK: Conversions
impl TryFrom<Vec<u8>> for AuthSessionRefreshTokenHash {
    type Error = AuthSessionError;

    fn try_from(value: Vec<u8>) -> Result<Self, Self::Error> {
        if value.is_empty() {
            return Err(AuthSessionError::RefreshTokenHashEmpty);
        }

        Ok(Self(value))
    }
}

impl From<AuthSessionRefreshTokenHash> for Vec<u8> {
    fn from(vo: AuthSessionRefreshTokenHash) -> Self {
        vo.0
    }
}
