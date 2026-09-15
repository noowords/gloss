#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum SalonScheduleExceptionType {
    Closed,
    CustomHours
}

impl From<SalonScheduleExceptionType> for String {
    fn from(value: SalonScheduleExceptionType) -> Self {
        match value {
            SalonScheduleExceptionType::Closed => "closed",
            SalonScheduleExceptionType::CustomHours => "custom_hours"
        }.to_string()
    }
}

impl TryFrom<String> for SalonScheduleExceptionType {
    type Error = anyhow::Error;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        Self::try_from(value.as_str())
    }
}

impl TryFrom<&str> for SalonScheduleExceptionType {
    type Error = anyhow::Error;

    fn try_from(str: &str) -> Result<Self, Self::Error> {
        match str {
            "closed" => Ok(Self::Closed),
            "custom_hours" => Ok(Self::CustomHours),
            _ => anyhow::bail!("Invalid SalonScheduleExceptionType: {}", str)
        }
    }
}
