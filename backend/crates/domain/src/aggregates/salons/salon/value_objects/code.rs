#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SalonCode(String);

impl From<SalonCode> for String {
    fn from(value: SalonCode) -> Self {
        value.0
    }
}

impl TryFrom<String> for SalonCode {
    type Error = anyhow::Error;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        if value.chars().count() > 32 {
            anyhow::bail!("Invalid SalonCode");
        }

        Ok(Self(value))
    }
}

impl TryFrom<&str> for SalonCode {
    type Error = anyhow::Error;

    fn try_from(str: &str) -> Result<Self, Self::Error> {
        str.to_string().try_into()
    }
}
