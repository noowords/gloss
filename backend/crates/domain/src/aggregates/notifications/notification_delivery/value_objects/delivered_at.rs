use chrono::{ NaiveDateTime };

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct NotificationDeliveryDeliveredAt(NaiveDateTime);

impl From<NotificationDeliveryDeliveredAt> for NaiveDateTime { fn from(value: NotificationDeliveryDeliveredAt) -> Self { value.0 } }
impl From<NaiveDateTime> for NotificationDeliveryDeliveredAt { fn from(value: NaiveDateTime) -> Self { Self(value) } }
