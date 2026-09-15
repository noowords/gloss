#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NotificationDeliveryChannel(String);

impl From<NotificationDeliveryChannel> for String {
    fn from(value: NotificationDeliveryChannel) -> Self { value.0 }
}

impl TryFrom<String> for NotificationDeliveryChannel {
    type Error = anyhow::Error;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        if value.chars().count() > 32 { anyhow::bail!("Invalid NotificationDeliveryChannel"); }
        Ok(Self(value))
    }
}

impl TryFrom<&str> for NotificationDeliveryChannel {
    type Error = anyhow::Error;

    fn try_from(str: &str) -> Result<Self, Self::Error> { str.to_string().try_into() }
}
