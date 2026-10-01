use chrono::NaiveDateTime;

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct NotificationCreatedAt(NaiveDateTime);

// MARK: Conversions
impl From<NaiveDateTime> for NotificationCreatedAt {
    fn from(value: NaiveDateTime) -> Self {
        Self(value)
    }
}

impl From<NotificationCreatedAt> for NaiveDateTime {
    fn from(vo: NotificationCreatedAt) -> Self {
        vo.0
    }
}
