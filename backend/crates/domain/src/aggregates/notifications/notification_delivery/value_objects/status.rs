#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum NotificationDeliveryStatus {
    Pending,
    Processing,
    Delivered,
    Failed
}

impl From<NotificationDeliveryStatus> for String {
    fn from(value: NotificationDeliveryStatus) -> Self {
        match value {
            NotificationDeliveryStatus::Pending => "pending",
            NotificationDeliveryStatus::Processing => "processing",
            NotificationDeliveryStatus::Delivered => "delivered",
            NotificationDeliveryStatus::Failed => "failed"
        }.to_string()
    }
}

impl TryFrom<String> for NotificationDeliveryStatus {
    type Error = anyhow::Error;

    fn try_from(value: String) -> Result<Self, Self::Error> { Self::try_from(value.as_str()) }
}

impl TryFrom<&str> for NotificationDeliveryStatus {
    type Error = anyhow::Error;

    fn try_from(str: &str) -> Result<Self, Self::Error> {
        match str {
            "pending" => Ok(Self::Pending),
            "processing" => Ok(Self::Processing),
            "delivered" => Ok(Self::Delivered),
            "failed" => Ok(Self::Failed),
            _ => anyhow::bail!("Invalid NotificationDeliveryStatus: {}", str)
        }
    }
}
