use chrono::NaiveDateTime;

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct NotificationDeliveryDeliveredAt(NaiveDateTime);

// MARK: Conversions
impl From<NaiveDateTime> for NotificationDeliveryDeliveredAt {
    fn from(value: NaiveDateTime) -> Self {
        Self(value)
    }
}

impl From<NotificationDeliveryDeliveredAt> for NaiveDateTime {
    fn from(vo: NotificationDeliveryDeliveredAt) -> Self {
        vo.0
    }
}
