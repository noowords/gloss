use uuid::{ Uuid };

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct SalonScheduleExceptionIntervalId(Uuid);

impl SalonScheduleExceptionIntervalId {
    pub fn generate() -> Self {
        Self(Uuid::now_v7())
    }
}

impl From<SalonScheduleExceptionIntervalId> for Uuid {
    fn from(id: SalonScheduleExceptionIntervalId) -> Self {
        id.0
    }
}

impl From<SalonScheduleExceptionIntervalId> for String {
    fn from(id: SalonScheduleExceptionIntervalId) -> Self {
        id.0.to_string()
    }
}

impl From<SalonScheduleExceptionIntervalId> for [u8; 16] {
    fn from(id: SalonScheduleExceptionIntervalId) -> Self {
        id.0.into_bytes()
    }
}

impl From<Uuid> for SalonScheduleExceptionIntervalId {
    fn from(uuid: Uuid) -> Self {
        Self(uuid)
    }
}

impl TryFrom<&str> for SalonScheduleExceptionIntervalId {
    type Error = anyhow::Error;

    fn try_from(str: &str) -> Result<Self, Self::Error> {
        Uuid::parse_str(str)
            .map(SalonScheduleExceptionIntervalId)
            .map_err(|_| anyhow::anyhow!("Invalid SalonScheduleExceptionIntervalId: {}", str))
    }
}

impl TryFrom<&[u8; 16]> for SalonScheduleExceptionIntervalId {
    type Error = anyhow::Error;

    fn try_from(bytes: &[u8; 16]) -> Result<Self, Self::Error> {
        Uuid::from_slice(bytes)
            .map(SalonScheduleExceptionIntervalId)
            .map_err(|_| anyhow::anyhow!("Invalid SalonScheduleExceptionIntervalId: {}", String::from_utf8_lossy(bytes)))
    }
}
