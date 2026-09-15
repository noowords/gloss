use uuid::{ Uuid };

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct UserProviderId(Uuid);

impl UserProviderId {
    pub fn generate() -> Self {
        Self(Uuid::now_v7())
    }
}

impl From<UserProviderId> for Uuid {
    fn from(id: UserProviderId) -> Self {
        id.0
    }
}

impl From<UserProviderId> for String {
    fn from(id: UserProviderId) -> Self {
        id.0.to_string()
    }
}

impl From<UserProviderId> for [u8; 16] {
    fn from(id: UserProviderId) -> Self {
        id.0.into_bytes()
    }
}

impl From<Uuid> for UserProviderId {
    fn from(uuid: Uuid) -> Self {
        Self(uuid)
    }
}

impl TryFrom<&str> for UserProviderId {
    type Error = anyhow::Error;

    fn try_from(str: &str) -> Result<Self, Self::Error> {
        Uuid::parse_str(str)
            .map(UserProviderId)
            .map_err(|_| anyhow::anyhow!("Invalid UserProviderId: {}", str))
    }
}

impl TryFrom<&[u8; 16]> for UserProviderId {
    type Error = anyhow::Error;

    fn try_from(bytes: &[u8; 16]) -> Result<Self, Self::Error> {
        Uuid::from_slice(bytes)
            .map(UserProviderId)
            .map_err(|_| anyhow::anyhow!("Invalid UserProviderId: {}", String::from_utf8_lossy(bytes)))
    }
}
