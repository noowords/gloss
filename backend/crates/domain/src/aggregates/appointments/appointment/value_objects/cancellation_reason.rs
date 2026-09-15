#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AppointmentCancellationReason(String);

impl From<AppointmentCancellationReason> for String {
    fn from(value: AppointmentCancellationReason) -> Self {
        value.0
    }
}

impl TryFrom<String> for AppointmentCancellationReason {
    type Error = anyhow::Error;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        if value.chars().count() > 255 {
            anyhow::bail!("Invalid AppointmentCancellationReason");
        }

        Ok(Self(value))
    }
}

impl TryFrom<&str> for AppointmentCancellationReason {
    type Error = anyhow::Error;

    fn try_from(str: &str) -> Result<Self, Self::Error> {
        str.to_string().try_into()
    }
}
