use uuid::{ Uuid };

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct SalonScheduleExceptionId(Uuid);

impl SalonScheduleExceptionId {
    pub fn generate() -> Self {
        Self(Uuid::now_v7())
    }
}

impl From<SalonScheduleExceptionId> for Uuid {
    fn from(id: SalonScheduleExceptionId) -> Self {
        id.0
    }
}

impl From<SalonScheduleExceptionId> for String {
    fn from(id: SalonScheduleExceptionId) -> Self {
        id.0.to_string()
    }
}

impl From<SalonScheduleExceptionId> for [u8; 16] {
    fn from(id: SalonScheduleExceptionId) -> Self {
        id.0.into_bytes()
    }
}

impl From<Uuid> for SalonScheduleExceptionId {
    fn from(uuid: Uuid) -> Self {
        Self(uuid)
    }
}

impl TryFrom<&str> for SalonScheduleExceptionId {
    type Error = anyhow::Error;

    fn try_from(str: &str) -> Result<Self, Self::Error> {
        Uuid::parse_str(str)
            .map(SalonScheduleExceptionId)
            .map_err(|_| anyhow::anyhow!("Invalid SalonScheduleExceptionId: {}", str))
    }
}

impl TryFrom<&[u8; 16]> for SalonScheduleExceptionId {
    type Error = anyhow::Error;

    fn try_from(bytes: &[u8; 16]) -> Result<Self, Self::Error> {
        Uuid::from_slice(bytes)
            .map(SalonScheduleExceptionId)
            .map_err(|_| anyhow::anyhow!("Invalid SalonScheduleExceptionId: {}", String::from_utf8_lossy(bytes)))
    }
}
