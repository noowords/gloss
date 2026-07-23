use uuid::{ Uuid };

#[derive(Copy, Clone, Eq, PartialEq)]
pub struct AppointmentId(Uuid);

impl AppointmentId {
    pub fn generate() -> Self {
        Self(Uuid::now_v7())
    }

    pub fn uuid(&self) -> Uuid {
        self.0
    }
}

impl From<AppointmentId> for Uuid {
    fn from(id: AppointmentId) -> Self {
        id.0
    }
}

impl From<AppointmentId> for String {
    fn from(id: AppointmentId) -> Self {
        id.uuid().to_string()
    }
}

impl From<AppointmentId> for [u8; 16] {
    fn from(id: AppointmentId) -> Self {
        id.uuid().into_bytes()
    }
}

impl From<Uuid> for AppointmentId {
    fn from(uuid: Uuid) -> Self {
        Self(uuid)
    }
}

impl TryFrom<&str> for AppointmentId {
    type Error = anyhow::Error;

    fn try_from(str: &str) -> Result<Self, Self::Error> {
        Uuid::parse_str(str)
            .map(AppointmentId)
            .map_err(|_| anyhow::anyhow!("Invalid AppointmentId: {}", str))
    }
}

impl TryFrom<&[u8; 16]> for AppointmentId {
    type Error = anyhow::Error;

    fn try_from(bytes: &[u8; 16]) -> Result<Self, Self::Error> {
        Uuid::from_slice(bytes)
            .map(AppointmentId)
            .map_err(|_| anyhow::anyhow!("Invalid AppointmentId: {}", String::from_utf8_lossy(bytes)))
    }
}
