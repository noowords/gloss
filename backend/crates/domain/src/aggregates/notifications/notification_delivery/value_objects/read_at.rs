use chrono::{ NaiveDateTime };

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct NotificationDeliveryReadAt(NaiveDateTime);

impl From<NotificationDeliveryReadAt> for NaiveDateTime { fn from(value: NotificationDeliveryReadAt) -> Self { value.0 } }
impl From<NaiveDateTime> for NotificationDeliveryReadAt { fn from(value: NaiveDateTime) -> Self { Self(value) } }
