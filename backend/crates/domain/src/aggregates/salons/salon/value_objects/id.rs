use uuid::{ Uuid };

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct SalonId(Uuid);

impl SalonId {
    pub fn generate() -> Self {
        Self(Uuid::now_v7())
    }
}

impl From<SalonId> for Uuid {
    fn from(id: SalonId) -> Self {
        id.0
    }
}

impl From<SalonId> for String {
    fn from(id: SalonId) -> Self {
        id.0.to_string()
    }
}

impl From<SalonId> for [u8; 16] {
    fn from(id: SalonId) -> Self {
        id.0.into_bytes()
    }
}

impl From<Uuid> for SalonId {
    fn from(uuid: Uuid) -> Self {
        Self(uuid)
    }
}

impl TryFrom<&str> for SalonId {
    type Error = anyhow::Error;

    fn try_from(str: &str) -> Result<Self, Self::Error> {
        Uuid::parse_str(str)
            .map(SalonId)
            .map_err(|_| anyhow::anyhow!("Invalid SalonId: {}", str))
    }
}

impl TryFrom<&[u8; 16]> for SalonId {
    type Error = anyhow::Error;

    fn try_from(bytes: &[u8; 16]) -> Result<Self, Self::Error> {
        Uuid::from_slice(bytes)
            .map(SalonId)
            .map_err(|_| anyhow::anyhow!("Invalid SalonId: {}", String::from_utf8_lossy(bytes)))
    }
}
