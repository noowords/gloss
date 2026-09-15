#[derive(Debug, Clone, PartialEq, Eq)]
pub struct UserProviderSubject(String);

impl From<UserProviderSubject> for String {
    fn from(value: UserProviderSubject) -> Self {
        value.0
    }
}

impl TryFrom<String> for UserProviderSubject {
    type Error = anyhow::Error;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        if value.chars().count() > 255 {
            anyhow::bail!("Invalid UserProviderSubject");
        }

        Ok(Self(value))
    }
}

impl TryFrom<&str> for UserProviderSubject {
    type Error = anyhow::Error;

    fn try_from(str: &str) -> Result<Self, Self::Error> {
        str.to_string().try_into()
    }
}
