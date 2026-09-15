use uuid::{ Uuid };

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct NotificationId(Uuid);

impl NotificationId {
    pub fn generate() -> Self {
        Self(Uuid::now_v7())
    }
}

impl From<NotificationId> for Uuid {
    fn from(id: NotificationId) -> Self {
        id.0
    }
}

impl From<NotificationId> for String {
    fn from(id: NotificationId) -> Self {
        id.0.to_string()
    }
}

impl From<NotificationId> for [u8; 16] {
    fn from(id: NotificationId) -> Self {
        id.0.into_bytes()
    }
}

impl From<Uuid> for NotificationId {
    fn from(uuid: Uuid) -> Self {
        Self(uuid)
    }
}

impl TryFrom<&str> for NotificationId {
    type Error = anyhow::Error;

    fn try_from(str: &str) -> Result<Self, Self::Error> {
        Uuid::parse_str(str)
            .map(NotificationId)
            .map_err(|_| anyhow::anyhow!("Invalid NotificationId: {}", str))
    }
}

impl TryFrom<&[u8; 16]> for NotificationId {
    type Error = anyhow::Error;

    fn try_from(bytes: &[u8; 16]) -> Result<Self, Self::Error> {
        Uuid::from_slice(bytes)
            .map(NotificationId)
            .map_err(|_| anyhow::anyhow!("Invalid NotificationId: {}", String::from_utf8_lossy(bytes)))
    }
}
