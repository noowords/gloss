#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NotificationDeliveryError(String);

impl From<NotificationDeliveryError> for String { fn from(value: NotificationDeliveryError) -> Self { value.0 } }

impl TryFrom<String> for NotificationDeliveryError {
    type Error = anyhow::Error;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        if value.chars().count() > 512 { anyhow::bail!("Invalid NotificationDeliveryError"); }
        Ok(Self(value))
    }
}

impl TryFrom<&str> for NotificationDeliveryError {
    type Error = anyhow::Error;

    fn try_from(str: &str) -> Result<Self, Self::Error> { str.to_string().try_into() }
}
