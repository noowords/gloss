use uuid::{ Uuid };

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct SpecialistId(Uuid);

impl SpecialistId {
    pub fn generate() -> Self {
        Self(Uuid::now_v7())
    }
}

impl From<SpecialistId> for Uuid {
    fn from(id: SpecialistId) -> Self {
        id.0
    }
}

impl From<SpecialistId> for String {
    fn from(id: SpecialistId) -> Self {
        id.0.to_string()
    }
}

impl From<SpecialistId> for [u8; 16] {
    fn from(id: SpecialistId) -> Self {
        id.0.into_bytes()
    }
}

impl From<Uuid> for SpecialistId {
    fn from(uuid: Uuid) -> Self {
        Self(uuid)
    }
}

impl TryFrom<&str> for SpecialistId {
    type Error = anyhow::Error;

    fn try_from(str: &str) -> Result<Self, Self::Error> {
        Uuid::parse_str(str)
            .map(SpecialistId)
            .map_err(|_| anyhow::anyhow!("Invalid SpecialistId: {}", str))
    }
}

impl TryFrom<&[u8; 16]> for SpecialistId {
    type Error = anyhow::Error;

    fn try_from(bytes: &[u8; 16]) -> Result<Self, Self::Error> {
        Uuid::from_slice(bytes)
            .map(SpecialistId)
            .map_err(|_| anyhow::anyhow!("Invalid SpecialistId: {}", String::from_utf8_lossy(bytes)))
    }
}
