#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ServiceName(String);

impl From<ServiceName> for String {
    fn from(value: ServiceName) -> Self {
        value.0
    }
}

impl TryFrom<String> for ServiceName {
    type Error = anyhow::Error;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        if value.chars().count() > 128 {
            anyhow::bail!("Invalid ServiceName");
        }

        Ok(Self(value))
    }
}

impl TryFrom<&str> for ServiceName {
    type Error = anyhow::Error;

    fn try_from(str: &str) -> Result<Self, Self::Error> {
        str.to_string().try_into()
    }
}
