#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ServicePreviewUrl(String);

impl From<ServicePreviewUrl> for String {
    fn from(value: ServicePreviewUrl) -> Self {
        value.0
    }
}

impl TryFrom<String> for ServicePreviewUrl {
    type Error = anyhow::Error;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        if value.chars().count() > 512 {
            anyhow::bail!("Invalid ServicePreviewUrl");
        }

        Ok(Self(value))
    }
}

impl TryFrom<&str> for ServicePreviewUrl {
    type Error = anyhow::Error;

    fn try_from(str: &str) -> Result<Self, Self::Error> {
        str.to_string().try_into()
    }
}
