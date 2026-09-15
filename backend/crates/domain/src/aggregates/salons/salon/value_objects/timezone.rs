#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SalonTimezone(String);

impl From<SalonTimezone> for String {
    fn from(value: SalonTimezone) -> Self {
        value.0
    }
}

impl TryFrom<String> for SalonTimezone {
    type Error = anyhow::Error;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        if value.chars().count() > 64 {
            anyhow::bail!("Invalid SalonTimezone");
        }

        value.parse::<chrono_tz::Tz>()
            .map_err(|_| anyhow::anyhow!("Invalid IANA timezone: {}", value))?;

        Ok(Self(value))
    }
}

impl TryFrom<&str> for SalonTimezone {
    type Error = anyhow::Error;

    fn try_from(str: &str) -> Result<Self, Self::Error> {
        str.to_string().try_into()
    }
}
