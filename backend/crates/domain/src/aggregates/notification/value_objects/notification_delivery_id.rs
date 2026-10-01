use uuid::Uuid;

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct NotificationDeliveryId(Uuid);

// MARK: Constructors
impl NotificationDeliveryId {
    pub fn generate() -> Self {
        Self(Uuid::now_v7())
    }
}

// MARK: Conversions
impl From<Uuid> for NotificationDeliveryId {
    fn from(value: Uuid) -> Self {
        Self(value)
    }
}

impl From<NotificationDeliveryId> for Uuid {
    fn from(vo: NotificationDeliveryId) -> Self {
        vo.0
    }
}
