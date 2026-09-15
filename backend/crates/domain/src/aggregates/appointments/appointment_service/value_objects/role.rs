#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum AppointmentServiceRole {
    Primary,
    Addon
}

impl From<AppointmentServiceRole> for String {
    fn from(value: AppointmentServiceRole) -> Self {
        match value {
            AppointmentServiceRole::Primary => "primary",
            AppointmentServiceRole::Addon => "addon"
        }.to_string()
    }
}

impl TryFrom<String> for AppointmentServiceRole {
    type Error = anyhow::Error;

    fn try_from(str: String) -> Result<Self, Self::Error> {
        Self::try_from(str.as_str())
    }
}

impl TryFrom<&str> for AppointmentServiceRole {
    type Error = anyhow::Error;

    fn try_from(str: &str) -> Result<Self, Self::Error> {
        match str {
            "primary" => Ok(AppointmentServiceRole::Primary),
            "addon" => Ok(AppointmentServiceRole::Addon),
            _ => anyhow::bail!("Invalid AppointmentServiceRole: {}", str)
        }
    }
}
