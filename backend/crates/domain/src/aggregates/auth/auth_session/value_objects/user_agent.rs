#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AuthSessionUserAgent(String);

impl From<AuthSessionUserAgent> for String {
    fn from(value: AuthSessionUserAgent) -> Self {
        value.0
    }
}

impl TryFrom<String> for AuthSessionUserAgent {
    type Error = anyhow::Error;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        if value.chars().count() > 512 {
            anyhow::bail!("Invalid AuthSessionUserAgent");
        }

        Ok(Self(value))
    }
}

impl TryFrom<&str> for AuthSessionUserAgent {
    type Error = anyhow::Error;

    fn try_from(str: &str) -> Result<Self, Self::Error> {
        str.to_string().try_into()
    }
}
