#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum AppointmentStatus {
    Scheduled,
    Cancelled,
    Completed,
    NoShow
}

impl From<AppointmentStatus> for String {
    fn from(value: AppointmentStatus) -> Self {
        match value {
            AppointmentStatus::Scheduled => "scheduled",
            AppointmentStatus::Cancelled => "cancelled",
            AppointmentStatus::Completed => "completed",
            AppointmentStatus::NoShow => "no_show"
        }.to_string()
    }
}

impl TryFrom<String> for AppointmentStatus {
    type Error = anyhow::Error;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        Self::try_from(value.as_str())
    }
}

impl TryFrom<&str> for AppointmentStatus {
    type Error = anyhow::Error;

    fn try_from(str: &str) -> Result<Self, Self::Error> {
        match str {
            "scheduled" => Ok(Self::Scheduled),
            "cancelled" => Ok(Self::Cancelled),
            "completed" => Ok(Self::Completed),
            "no_show" => Ok(Self::NoShow),
            _ => anyhow::bail!("Invalid AppointmentStatus: {}", str)
        }
    }
}
