use super::super::errors::NotificationError;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NotificationTitle(String);

// MARK: Conversions
impl TryFrom<String> for NotificationTitle {
    type Error = NotificationError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        let value = value.trim();

        if value.is_empty() {
            return Err(NotificationError::TitleEmpty);
        }

        if value.chars().count() > 128 {
            return Err(NotificationError::TitleTooLong);
        }

        Ok(Self(value.to_owned()))
    }
}

impl From<NotificationTitle> for String {
    fn from(vo: NotificationTitle) -> Self {
        vo.0
    }
}
