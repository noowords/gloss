#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NotificationMessage(String);

impl From<NotificationMessage> for String {
    fn from(value: NotificationMessage) -> Self {
        value.0
    }
}

impl TryFrom<String> for NotificationMessage {
    type Error = anyhow::Error;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        if value.chars().count() > 1024 {
            anyhow::bail!("Invalid NotificationMessage");
        }

        Ok(Self(value))
    }
}

impl TryFrom<&str> for NotificationMessage {
    type Error = anyhow::Error;

    fn try_from(str: &str) -> Result<Self, Self::Error> {
        str.to_string().try_into()
    }
}
