use uuid::{ Uuid };

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct ServiceId(Uuid);

impl ServiceId {
    pub fn generate() -> Self {
        Self(Uuid::now_v7())
    }
}

impl From<ServiceId> for Uuid {
    fn from(id: ServiceId) -> Self {
        id.0
    }
}

impl From<ServiceId> for String {
    fn from(id: ServiceId) -> Self {
        id.0.to_string()
    }
}

impl From<ServiceId> for [u8; 16] {
    fn from(id: ServiceId) -> Self {
        id.0.into_bytes()
    }
}

impl From<Uuid> for ServiceId {
    fn from(uuid: Uuid) -> Self {
        Self(uuid)
    }
}

impl TryFrom<&str> for ServiceId {
    type Error = anyhow::Error;

    fn try_from(str: &str) -> Result<Self, Self::Error> {
        Uuid::parse_str(str)
            .map(ServiceId)
            .map_err(|_| anyhow::anyhow!("Invalid UserId: {}", str))
    }
}

impl TryFrom<&[u8; 16]> for ServiceId {
    type Error = anyhow::Error;

    fn try_from(bytes: &[u8; 16]) -> Result<Self, Self::Error> {
        Uuid::from_slice(bytes)
            .map(ServiceId)
            .map_err(|_| anyhow::anyhow!("Invalid ServiceId: {}", String::from_utf8_lossy(bytes)))
    }
}
