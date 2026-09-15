use uuid::{ Uuid };

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct SpecialistScheduleOverrideId(Uuid);

impl SpecialistScheduleOverrideId {
    pub fn generate() -> Self {
        Self(Uuid::now_v7())
    }
}

impl From<SpecialistScheduleOverrideId> for Uuid {
    fn from(id: SpecialistScheduleOverrideId) -> Self {
        id.0
    }
}

impl From<SpecialistScheduleOverrideId> for String {
    fn from(id: SpecialistScheduleOverrideId) -> Self {
        id.0.to_string()
    }
}

impl From<SpecialistScheduleOverrideId> for [u8; 16] {
    fn from(id: SpecialistScheduleOverrideId) -> Self {
        id.0.into_bytes()
    }
}

impl From<Uuid> for SpecialistScheduleOverrideId {
    fn from(uuid: Uuid) -> Self {
        Self(uuid)
    }
}

impl TryFrom<&str> for SpecialistScheduleOverrideId {
    type Error = anyhow::Error;

    fn try_from(str: &str) -> Result<Self, Self::Error> {
        Uuid::parse_str(str)
            .map(SpecialistScheduleOverrideId)
            .map_err(|_| anyhow::anyhow!("Invalid SpecialistScheduleOverrideId: {}", str))
    }
}

impl TryFrom<&[u8; 16]> for SpecialistScheduleOverrideId {
    type Error = anyhow::Error;

    fn try_from(bytes: &[u8; 16]) -> Result<Self, Self::Error> {
        Uuid::from_slice(bytes)
            .map(SpecialistScheduleOverrideId)
            .map_err(|_| anyhow::anyhow!("Invalid SpecialistScheduleOverrideId: {}", String::from_utf8_lossy(bytes)))
    }
}
