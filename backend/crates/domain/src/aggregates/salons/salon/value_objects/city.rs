#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SalonCity(String);

impl From<SalonCity> for String {
    fn from(value: SalonCity) -> Self {
        value.0
    }
}

impl TryFrom<String> for SalonCity {
    type Error = anyhow::Error;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        if value.chars().count() > 128 {
            anyhow::bail!("Invalid SalonCity");
        }

        Ok(Self(value))
    }
}

impl TryFrom<&str> for SalonCity {
    type Error = anyhow::Error;

    fn try_from(str: &str) -> Result<Self, Self::Error> {
        str.to_string().try_into()
    }
}
