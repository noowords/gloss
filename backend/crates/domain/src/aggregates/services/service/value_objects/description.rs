#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ServiceDescription(String);

impl From<ServiceDescription> for String {
    fn from(value: ServiceDescription) -> Self {
        value.0
    }
}

impl TryFrom<String> for ServiceDescription {
    type Error = anyhow::Error;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        if value.len() > 65535 {
            anyhow::bail!("Invalid ServiceDescription");
        }

        Ok(Self(value))
    }
}

impl TryFrom<&str> for ServiceDescription {
    type Error = anyhow::Error;

    fn try_from(str: &str) -> Result<Self, Self::Error> {
        str.to_string().try_into()
    }
}
