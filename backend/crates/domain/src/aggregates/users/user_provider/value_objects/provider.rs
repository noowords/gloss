#[derive(Debug, Clone, PartialEq, Eq)]
pub struct UserProviderProvider(String);

impl From<UserProviderProvider> for String {
    fn from(value: UserProviderProvider) -> Self {
        value.0
    }
}

impl TryFrom<String> for UserProviderProvider {
    type Error = anyhow::Error;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        if value.chars().count() > 32 {
            anyhow::bail!("Invalid UserProviderProvider");
        }

        Ok(Self(value))
    }
}

impl TryFrom<&str> for UserProviderProvider {
    type Error = anyhow::Error;

    fn try_from(str: &str) -> Result<Self, Self::Error> {
        str.to_string().try_into()
    }
}
