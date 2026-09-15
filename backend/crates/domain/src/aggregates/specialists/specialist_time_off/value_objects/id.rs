use uuid::{ Uuid };

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct SpecialistTimeOffId(Uuid);

impl SpecialistTimeOffId {
    pub fn generate() -> Self {
        Self(Uuid::now_v7())
    }
}

impl From<SpecialistTimeOffId> for Uuid {
    fn from(id: SpecialistTimeOffId) -> Self {
        id.0
    }
}

impl From<SpecialistTimeOffId> for String {
    fn from(id: SpecialistTimeOffId) -> Self {
        id.0.to_string()
    }
}

impl From<SpecialistTimeOffId> for [u8; 16] {
    fn from(id: SpecialistTimeOffId) -> Self {
        id.0.into_bytes()
    }
}

impl From<Uuid> for SpecialistTimeOffId {
    fn from(uuid: Uuid) -> Self {
        Self(uuid)
    }
}

impl TryFrom<&str> for SpecialistTimeOffId {
    type Error = anyhow::Error;

    fn try_from(str: &str) -> Result<Self, Self::Error> {
        Uuid::parse_str(str)
            .map(SpecialistTimeOffId)
            .map_err(|_| anyhow::anyhow!("Invalid SpecialistTimeOffId: {}", str))
    }
}

impl TryFrom<&[u8; 16]> for SpecialistTimeOffId {
    type Error = anyhow::Error;

    fn try_from(bytes: &[u8; 16]) -> Result<Self, Self::Error> {
        Uuid::from_slice(bytes)
            .map(SpecialistTimeOffId)
            .map_err(|_| anyhow::anyhow!("Invalid SpecialistTimeOffId: {}", String::from_utf8_lossy(bytes)))
    }
}
