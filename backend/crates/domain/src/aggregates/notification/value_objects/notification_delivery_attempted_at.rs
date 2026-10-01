use chrono::NaiveDateTime;

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct NotificationDeliveryAttemptedAt(NaiveDateTime);

// MARK: Conversions
impl From<NaiveDateTime> for NotificationDeliveryAttemptedAt {
    fn from(value: NaiveDateTime) -> Self {
        Self(value)
    }
}

impl From<NotificationDeliveryAttemptedAt> for NaiveDateTime {
    fn from(vo: NotificationDeliveryAttemptedAt) -> Self {
        vo.0
    }
}
