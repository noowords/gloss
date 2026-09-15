use uuid::{ Uuid };

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct SpecialistScheduleIntervalId(Uuid);

impl SpecialistScheduleIntervalId {
    pub fn generate() -> Self {
        Self(Uuid::now_v7())
    }
}

impl From<SpecialistScheduleIntervalId> for Uuid {
    fn from(id: SpecialistScheduleIntervalId) -> Self {
        id.0
    }
}

impl From<SpecialistScheduleIntervalId> for String {
    fn from(id: SpecialistScheduleIntervalId) -> Self {
        id.0.to_string()
    }
}

impl From<SpecialistScheduleIntervalId> for [u8; 16] {
    fn from(id: SpecialistScheduleIntervalId) -> Self {
        id.0.into_bytes()
    }
}

impl From<Uuid> for SpecialistScheduleIntervalId {
    fn from(uuid: Uuid) -> Self {
        Self(uuid)
    }
}

impl TryFrom<&str> for SpecialistScheduleIntervalId {
    type Error = anyhow::Error;

    fn try_from(str: &str) -> Result<Self, Self::Error> {
        Uuid::parse_str(str)
            .map(SpecialistScheduleIntervalId)
            .map_err(|_| anyhow::anyhow!("Invalid SpecialistScheduleIntervalId: {}", str))
    }
}

impl TryFrom<&[u8; 16]> for SpecialistScheduleIntervalId {
    type Error = anyhow::Error;

    fn try_from(bytes: &[u8; 16]) -> Result<Self, Self::Error> {
        Uuid::from_slice(bytes)
            .map(SpecialistScheduleIntervalId)
            .map_err(|_| anyhow::anyhow!("Invalid SpecialistScheduleIntervalId: {}", String::from_utf8_lossy(bytes)))
    }
}
