use uuid::{ Uuid };

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct SpecialistScheduleId(Uuid);

impl SpecialistScheduleId {
    pub fn generate() -> Self {
        Self(Uuid::now_v7())
    }
}

impl From<SpecialistScheduleId> for Uuid {
    fn from(id: SpecialistScheduleId) -> Self {
        id.0
    }
}

impl From<SpecialistScheduleId> for String {
    fn from(id: SpecialistScheduleId) -> Self {
        id.0.to_string()
    }
}

impl From<SpecialistScheduleId> for [u8; 16] {
    fn from(id: SpecialistScheduleId) -> Self {
        id.0.into_bytes()
    }
}

impl From<Uuid> for SpecialistScheduleId {
    fn from(uuid: Uuid) -> Self {
        Self(uuid)
    }
}

impl TryFrom<&str> for SpecialistScheduleId {
    type Error = anyhow::Error;

    fn try_from(str: &str) -> Result<Self, Self::Error> {
        Uuid::parse_str(str)
            .map(SpecialistScheduleId)
            .map_err(|_| anyhow::anyhow!("Invalid SpecialistScheduleId: {}", str))
    }
}

impl TryFrom<&[u8; 16]> for SpecialistScheduleId {
    type Error = anyhow::Error;

    fn try_from(bytes: &[u8; 16]) -> Result<Self, Self::Error> {
        Uuid::from_slice(bytes)
            .map(SpecialistScheduleId)
            .map_err(|_| anyhow::anyhow!("Invalid SpecialistScheduleId: {}", String::from_utf8_lossy(bytes)))
    }
}
