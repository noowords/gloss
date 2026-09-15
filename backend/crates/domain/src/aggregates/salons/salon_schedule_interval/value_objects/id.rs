use uuid::{ Uuid };

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct SalonScheduleIntervalId(Uuid);

impl SalonScheduleIntervalId {
    pub fn generate() -> Self {
        Self(Uuid::now_v7())
    }
}

impl From<SalonScheduleIntervalId> for Uuid {
    fn from(id: SalonScheduleIntervalId) -> Self {
        id.0
    }
}

impl From<SalonScheduleIntervalId> for String {
    fn from(id: SalonScheduleIntervalId) -> Self {
        id.0.to_string()
    }
}

impl From<SalonScheduleIntervalId> for [u8; 16] {
    fn from(id: SalonScheduleIntervalId) -> Self {
        id.0.into_bytes()
    }
}

impl From<Uuid> for SalonScheduleIntervalId {
    fn from(uuid: Uuid) -> Self {
        Self(uuid)
    }
}

impl TryFrom<&str> for SalonScheduleIntervalId {
    type Error = anyhow::Error;

    fn try_from(str: &str) -> Result<Self, Self::Error> {
        Uuid::parse_str(str)
            .map(SalonScheduleIntervalId)
            .map_err(|_| anyhow::anyhow!("Invalid SalonScheduleIntervalId: {}", str))
    }
}

impl TryFrom<&[u8; 16]> for SalonScheduleIntervalId {
    type Error = anyhow::Error;

    fn try_from(bytes: &[u8; 16]) -> Result<Self, Self::Error> {
        Uuid::from_slice(bytes)
            .map(SalonScheduleIntervalId)
            .map_err(|_| anyhow::anyhow!("Invalid SalonScheduleIntervalId: {}", String::from_utf8_lossy(bytes)))
    }
}
