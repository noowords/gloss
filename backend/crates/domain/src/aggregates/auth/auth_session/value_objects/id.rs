use uuid::{ Uuid };

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct AuthSessionId(Uuid);

impl AuthSessionId {
    pub fn generate() -> Self {
        Self(Uuid::now_v7())
    }
}

impl From<AuthSessionId> for Uuid {
    fn from(id: AuthSessionId) -> Self {
        id.0
    }
}

impl From<AuthSessionId> for String {
    fn from(id: AuthSessionId) -> Self {
        id.0.to_string()
    }
}

impl From<AuthSessionId> for [u8; 16] {
    fn from(id: AuthSessionId) -> Self {
        id.0.into_bytes()
    }
}

impl From<Uuid> for AuthSessionId {
    fn from(uuid: Uuid) -> Self {
        Self(uuid)
    }
}

impl TryFrom<&str> for AuthSessionId {
    type Error = anyhow::Error;

    fn try_from(str: &str) -> Result<Self, Self::Error> {
        Uuid::parse_str(str)
            .map(AuthSessionId)
            .map_err(|_| anyhow::anyhow!("Invalid AuthSessionId: {}", str))
    }
}

impl TryFrom<&[u8; 16]> for AuthSessionId {
    type Error = anyhow::Error;

    fn try_from(bytes: &[u8; 16]) -> Result<Self, Self::Error> {
        Uuid::from_slice(bytes)
            .map(AuthSessionId)
            .map_err(|_| anyhow::anyhow!("Invalid AuthSessionId: {}", String::from_utf8_lossy(bytes)))
    }
}
