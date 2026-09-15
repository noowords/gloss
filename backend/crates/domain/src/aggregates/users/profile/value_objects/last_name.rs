#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ProfileLastName(String);

impl From<ProfileLastName> for String {
    fn from(value: ProfileLastName) -> Self {
        value.0
    }
}

impl TryFrom<String> for ProfileLastName {
    type Error = anyhow::Error;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        if value.chars().count() > 128 {
            anyhow::bail!("Invalid ProfileLastName");
        }

        Ok(Self(value))
    }
}

impl TryFrom<&str> for ProfileLastName {
    type Error = anyhow::Error;

    fn try_from(str: &str) -> Result<Self, Self::Error> {
        str.to_string().try_into()
    }
}
