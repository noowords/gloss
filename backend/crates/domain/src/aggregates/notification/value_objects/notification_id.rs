use uuid::Uuid;

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct NotificationId(Uuid);

// MARK: Constructors
impl NotificationId {
    pub fn generate() -> Self {
        Self(Uuid::now_v7())
    }
}

// MARK: Conversions
impl From<Uuid> for NotificationId {
    fn from(value: Uuid) -> Self {
        Self(value)
    }
}

impl From<NotificationId> for Uuid {
    fn from(vo: NotificationId) -> Self {
        vo.0
    }
}
