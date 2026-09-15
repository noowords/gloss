#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SalonName(String);

impl From<SalonName> for String {
    fn from(value: SalonName) -> Self {
        value.0
    }
}

impl TryFrom<String> for SalonName {
    type Error = anyhow::Error;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        if value.chars().count() > 128 {
            anyhow::bail!("Invalid SalonName");
        }

        Ok(Self(value))
    }
}

impl TryFrom<&str> for SalonName {
    type Error = anyhow::Error;

    fn try_from(str: &str) -> Result<Self, Self::Error> {
        str.to_string().try_into()
    }
}
