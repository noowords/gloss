use uuid::{ Uuid };

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct SpecialistScheduleOverrideIntervalId(Uuid);

impl SpecialistScheduleOverrideIntervalId {
    pub fn generate() -> Self {
        Self(Uuid::now_v7())
    }
}

impl From<SpecialistScheduleOverrideIntervalId> for Uuid {
    fn from(id: SpecialistScheduleOverrideIntervalId) -> Self {
        id.0
    }
}

impl From<SpecialistScheduleOverrideIntervalId> for String {
    fn from(id: SpecialistScheduleOverrideIntervalId) -> Self {
        id.0.to_string()
    }
}

impl From<SpecialistScheduleOverrideIntervalId> for [u8; 16] {
    fn from(id: SpecialistScheduleOverrideIntervalId) -> Self {
        id.0.into_bytes()
    }
}

impl From<Uuid> for SpecialistScheduleOverrideIntervalId {
    fn from(uuid: Uuid) -> Self {
        Self(uuid)
    }
}

impl TryFrom<&str> for SpecialistScheduleOverrideIntervalId {
    type Error = anyhow::Error;

    fn try_from(str: &str) -> Result<Self, Self::Error> {
        Uuid::parse_str(str)
            .map(SpecialistScheduleOverrideIntervalId)
            .map_err(|_| anyhow::anyhow!("Invalid SpecialistScheduleOverrideIntervalId: {}", str))
    }
}

impl TryFrom<&[u8; 16]> for SpecialistScheduleOverrideIntervalId {
    type Error = anyhow::Error;

    fn try_from(bytes: &[u8; 16]) -> Result<Self, Self::Error> {
        Uuid::from_slice(bytes)
            .map(SpecialistScheduleOverrideIntervalId)
            .map_err(|_| anyhow::anyhow!("Invalid SpecialistScheduleOverrideIntervalId: {}", String::from_utf8_lossy(bytes)))
    }
}
