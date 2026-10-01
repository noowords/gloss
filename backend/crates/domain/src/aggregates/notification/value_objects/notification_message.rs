use super::super::errors::NotificationError;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NotificationMessage(String);

// MARK: Conversions
impl TryFrom<String> for NotificationMessage {
    type Error = NotificationError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        let value = value.trim();

        if value.is_empty() {
            return Err(NotificationError::MessageEmpty);
        }

        if value.chars().count() > 1024 {
            return Err(NotificationError::MessageTooLong);
        }

        Ok(Self(value.to_owned()))
    }
}

impl From<NotificationMessage> for String {
    fn from(vo: NotificationMessage) -> Self {
        vo.0
    }
}
