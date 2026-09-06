use uuid::{ Uuid };

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct UserId(Uuid);

impl UserId {
    pub fn generate() -> Self {
        Self(Uuid::now_v7())
    }
}

impl From<UserId> for Uuid {
    fn from(id: UserId) -> Self {
        id.0
    }
}

impl From<UserId> for String {
    fn from(id: UserId) -> Self {
        id.0.to_string()
    }
}

impl From<UserId> for [u8; 16] {
    fn from(id: UserId) -> Self {
        id.0.into_bytes()
    }
}

impl From<Uuid> for UserId {
    fn from(uuid: Uuid) -> Self {
        Self(uuid)
    }
}

impl TryFrom<&str> for UserId {
    type Error = anyhow::Error;

    fn try_from(str: &str) -> Result<Self, Self::Error> {
        Uuid::parse_str(str)
            .map(UserId)
            .map_err(|_| anyhow::anyhow!("Invalid UserId: {}", str))
    }
}

impl TryFrom<&[u8; 16]> for UserId {
    type Error = anyhow::Error;

    fn try_from(bytes: &[u8; 16]) -> Result<Self, Self::Error> {
        Uuid::from_slice(bytes)
            .map(UserId)
            .map_err(|_| anyhow::anyhow!("Invalid UserId: {}", String::from_utf8_lossy(bytes)))
    }
}
