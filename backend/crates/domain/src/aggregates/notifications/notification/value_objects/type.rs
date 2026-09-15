#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NotificationType(String);

impl From<NotificationType> for String {
    fn from(value: NotificationType) -> Self {
        value.0
    }
}

impl TryFrom<String> for NotificationType {
    type Error = anyhow::Error;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        if value.chars().count() > 64 {
            anyhow::bail!("Invalid NotificationType");
        }

        Ok(Self(value))
    }
}

impl TryFrom<&str> for NotificationType {
    type Error = anyhow::Error;

    fn try_from(str: &str) -> Result<Self, Self::Error> {
        str.to_string().try_into()
    }
}
