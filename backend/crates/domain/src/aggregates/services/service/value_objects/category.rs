#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ServiceCategory(String);

impl From<ServiceCategory> for String {
    fn from(value: ServiceCategory) -> Self {
        value.0
    }
}

impl TryFrom<String> for ServiceCategory {
    type Error = anyhow::Error;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        if value.chars().count() > 64 {
            anyhow::bail!("Invalid ServiceCategory");
        }

        Ok(Self(value))
    }
}

impl TryFrom<&str> for ServiceCategory {
    type Error = anyhow::Error;

    fn try_from(str: &str) -> Result<Self, Self::Error> {
        str.to_string().try_into()
    }
}
