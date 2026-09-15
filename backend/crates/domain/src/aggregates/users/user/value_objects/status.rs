#[derive(Debug, Clone, PartialEq, Eq)]
pub struct UserStatus(String);

impl From<UserStatus> for String {
    fn from(value: UserStatus) -> Self {
        value.0
    }
}

impl TryFrom<String> for UserStatus {
    type Error = anyhow::Error;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        if value.chars().count() > 32 {
            anyhow::bail!("Invalid UserStatus");
        }

        Ok(Self(value))
    }
}

impl TryFrom<&str> for UserStatus {
    type Error = anyhow::Error;

    fn try_from(str: &str) -> Result<Self, Self::Error> {
        str.to_string().try_into()
    }
}
