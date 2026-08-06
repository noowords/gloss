use uuid::{ Uuid };

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct UserIdentityId(Uuid);

impl UserIdentityId {
    pub fn generate() -> Self {
        Self(Uuid::now_v7())
    }
}

impl From<UserIdentityId> for Uuid {
    fn from(id: UserIdentityId) -> Self {
        id.0
    }
}

impl From<UserIdentityId> for String {
    fn from(id: UserIdentityId) -> Self {
        id.0.to_string()
    }
}

impl From<UserIdentityId> for [u8; 16] {
    fn from(id: UserIdentityId) -> Self {
        id.0.into_bytes()
    }
}

impl From<Uuid> for UserIdentityId {
    fn from(uuid: Uuid) -> Self {
        Self(uuid)
    }
}

impl TryFrom<&str> for UserIdentityId {
    type Error = anyhow::Error;
    
    fn try_from(str: &str) -> Result<Self, Self::Error> {
        Uuid::parse_str(str)
            .map(UserIdentityId)
            .map_err(|_| anyhow::anyhow!("Invalid UserId: {}", str))
    }
}

impl TryFrom<&[u8; 16]> for UserIdentityId {
    type Error = anyhow::Error;
    
    fn try_from(bytes: &[u8; 16]) -> Result<Self, Self::Error> {
        Uuid::from_slice(bytes)
            .map(UserIdentityId)
            .map_err(|_| anyhow::anyhow!("Invalid UserIdentityId: {}", String::from_utf8_lossy(bytes)))
    }
}
