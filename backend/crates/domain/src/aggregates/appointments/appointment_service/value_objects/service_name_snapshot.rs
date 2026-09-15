#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AppointmentServiceNameSnapshot(String);

impl From<AppointmentServiceNameSnapshot> for String {
    fn from(value: AppointmentServiceNameSnapshot) -> Self {
        value.0
    }
}

impl TryFrom<String> for AppointmentServiceNameSnapshot {
    type Error = anyhow::Error;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        if value.chars().count() > 128 {
            anyhow::bail!("Invalid AppointmentServiceNameSnapshot");
        }

        Ok(Self(value))
    }
}

impl TryFrom<&str> for AppointmentServiceNameSnapshot {
    type Error = anyhow::Error;

    fn try_from(str: &str) -> Result<Self, Self::Error> {
        str.to_string().try_into()
    }
}
