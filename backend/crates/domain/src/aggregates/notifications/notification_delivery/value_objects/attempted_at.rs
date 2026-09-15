use chrono::{ NaiveDateTime };

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct NotificationDeliveryAttemptedAt(NaiveDateTime);

impl From<NotificationDeliveryAttemptedAt> for NaiveDateTime { fn from(value: NotificationDeliveryAttemptedAt) -> Self { value.0 } }
impl From<NaiveDateTime> for NotificationDeliveryAttemptedAt { fn from(value: NaiveDateTime) -> Self { Self(value) } }
