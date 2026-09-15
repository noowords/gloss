#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AuthSessionIpAddress(Vec<u8>);

impl From<AuthSessionIpAddress> for Vec<u8> {
    fn from(value: AuthSessionIpAddress) -> Self {
        value.0
    }
}

impl TryFrom<Vec<u8>> for AuthSessionIpAddress {
    type Error = anyhow::Error;

    fn try_from(value: Vec<u8>) -> Result<Self, Self::Error> {
        if value.len() > 16 {
            anyhow::bail!("Invalid AuthSessionIpAddress");
        }

        Ok(Self(value))
    }
}
