use chrono::NaiveDateTime;

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct NotificationReadAt(NaiveDateTime);

// MARK: Conversions
impl From<NaiveDateTime> for NotificationReadAt {
    fn from(value: NaiveDateTime) -> Self {
        Self(value)
    }
}

impl From<NotificationReadAt> for NaiveDateTime {
    fn from(vo: NotificationReadAt) -> Self {
        vo.0
    }
}
