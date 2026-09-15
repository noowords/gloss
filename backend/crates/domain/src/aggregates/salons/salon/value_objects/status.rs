#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SalonStatus(String);

impl From<SalonStatus> for String {
    fn from(value: SalonStatus) -> Self {
        value.0
    }
}

impl TryFrom<String> for SalonStatus {
    type Error = anyhow::Error;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        if value.chars().count() > 32 {
            anyhow::bail!("Invalid SalonStatus");
        }

        Ok(Self(value))
    }
}

impl TryFrom<&str> for SalonStatus {
    type Error = anyhow::Error;

    fn try_from(str: &str) -> Result<Self, Self::Error> {
        str.to_string().try_into()
    }
}
