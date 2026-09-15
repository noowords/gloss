#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NotificationTitle(String);

impl From<NotificationTitle> for String {
    fn from(value: NotificationTitle) -> Self {
        value.0
    }
}

impl TryFrom<String> for NotificationTitle {
    type Error = anyhow::Error;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        if value.chars().count() > 128 {
            anyhow::bail!("Invalid NotificationTitle");
        }

        Ok(Self(value))
    }
}

impl TryFrom<&str> for NotificationTitle {
    type Error = anyhow::Error;

    fn try_from(str: &str) -> Result<Self, Self::Error> {
        str.to_string().try_into()
    }
}
