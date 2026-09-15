#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AuthSessionRefreshTokenHash(Vec<u8>);

impl From<AuthSessionRefreshTokenHash> for Vec<u8> {
    fn from(value: AuthSessionRefreshTokenHash) -> Self {
        value.0
    }
}

impl TryFrom<Vec<u8>> for AuthSessionRefreshTokenHash {
    type Error = anyhow::Error;

    fn try_from(value: Vec<u8>) -> Result<Self, Self::Error> {
        if value.len() > 64 {
            anyhow::bail!("Invalid AuthSessionRefreshTokenHash");
        }

        Ok(Self(value))
    }
}
