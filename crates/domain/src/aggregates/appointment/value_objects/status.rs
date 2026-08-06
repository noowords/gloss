#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum AppointmentStatus {
    Pending,
    Confirmed,
    Cancelled,
    Completed
}

impl From<AppointmentStatus> for String {
    fn from(status: AppointmentStatus) -> Self {
        match status {
            AppointmentStatus::Pending => "pending",
            AppointmentStatus::Confirmed => "confirmed",
            AppointmentStatus::Cancelled => "cancelled",
            AppointmentStatus::Completed => "completed"
        }.to_string()
    }
}

impl TryFrom<String> for AppointmentStatus {
    type Error = anyhow::Error;
    
    fn try_from(str: String) -> Result<Self, Self::Error> {
        match str.as_str() {
            "pending" => Ok(AppointmentStatus::Pending),
            "confirmed" => Ok(AppointmentStatus::Confirmed),
            "cancelled" => Ok(AppointmentStatus::Cancelled),
            "completed" => Ok(AppointmentStatus::Completed),
            _ => anyhow::bail!("Invalid AppointmentStatus: {}", str.to_string())
        }
    }
}

impl TryFrom<&str> for AppointmentStatus {
    type Error = anyhow::Error;
    
    fn try_from(str: &str) -> Result<Self, Self::Error> {
        match str {
            "pending" => Ok(AppointmentStatus::Pending),
            "confirmed" => Ok(AppointmentStatus::Confirmed),
            "cancelled" => Ok(AppointmentStatus::Cancelled),
            "completed" => Ok(AppointmentStatus::Completed),
            _ => anyhow::bail!("Invalid AppointmentStatus: {}", str.to_string())
        }
    }
}
