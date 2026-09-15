#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SalonScheduleExceptionReason(String);

impl From<SalonScheduleExceptionReason> for String {
    fn from(value: SalonScheduleExceptionReason) -> Self {
        value.0
    }
}

impl TryFrom<String> for SalonScheduleExceptionReason {
    type Error = anyhow::Error;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        if value.chars().count() > 255 {
            anyhow::bail!("Invalid SalonScheduleExceptionReason");
        }

        Ok(Self(value))
    }
}

impl TryFrom<&str> for SalonScheduleExceptionReason {
    type Error = anyhow::Error;

    fn try_from(str: &str) -> Result<Self, Self::Error> {
        str.to_string().try_into()
    }
}
