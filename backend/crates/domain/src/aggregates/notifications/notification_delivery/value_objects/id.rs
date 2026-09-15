use uuid::{ Uuid };

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct NotificationDeliveryId(Uuid);

impl NotificationDeliveryId {
    pub fn generate() -> Self {
        Self(Uuid::now_v7())
    }
}

impl From<NotificationDeliveryId> for Uuid {
    fn from(id: NotificationDeliveryId) -> Self { id.0 }
}

impl From<NotificationDeliveryId> for String {
    fn from(id: NotificationDeliveryId) -> Self { id.0.to_string() }
}

impl From<NotificationDeliveryId> for [u8; 16] {
    fn from(id: NotificationDeliveryId) -> Self { id.0.into_bytes() }
}

impl From<Uuid> for NotificationDeliveryId {
    fn from(uuid: Uuid) -> Self { Self(uuid) }
}

impl TryFrom<&str> for NotificationDeliveryId {
    type Error = anyhow::Error;

    fn try_from(str: &str) -> Result<Self, Self::Error> {
        Uuid::parse_str(str)
            .map(Self)
            .map_err(|_| anyhow::anyhow!("Invalid NotificationDeliveryId: {}", str))
    }
}

impl TryFrom<&[u8; 16]> for NotificationDeliveryId {
    type Error = anyhow::Error;

    fn try_from(bytes: &[u8; 16]) -> Result<Self, Self::Error> {
        Uuid::from_slice(bytes)
            .map(Self)
            .map_err(|_| anyhow::anyhow!("Invalid NotificationDeliveryId: {}", String::from_utf8_lossy(bytes)))
    }
}
