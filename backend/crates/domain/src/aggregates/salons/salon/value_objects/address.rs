#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SalonAddress(String);

impl From<SalonAddress> for String {
    fn from(value: SalonAddress) -> Self {
        value.0
    }
}

impl TryFrom<String> for SalonAddress {
    type Error = anyhow::Error;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        if value.chars().count() > 255 {
            anyhow::bail!("Invalid SalonAddress");
        }

        Ok(Self(value))
    }
}

impl TryFrom<&str> for SalonAddress {
    type Error = anyhow::Error;

    fn try_from(str: &str) -> Result<Self, Self::Error> {
        str.to_string().try_into()
    }
}
