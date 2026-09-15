#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ProfileFirstName(String);

impl From<ProfileFirstName> for String {
    fn from(value: ProfileFirstName) -> Self {
        value.0
    }
}

impl TryFrom<String> for ProfileFirstName {
    type Error = anyhow::Error;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        if value.chars().count() > 128 {
            anyhow::bail!("Invalid ProfileFirstName");
        }

        Ok(Self(value))
    }
}

impl TryFrom<&str> for ProfileFirstName {
    type Error = anyhow::Error;

    fn try_from(str: &str) -> Result<Self, Self::Error> {
        str.to_string().try_into()
    }
}
