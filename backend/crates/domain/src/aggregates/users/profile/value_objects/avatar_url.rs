#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ProfileAvatarUrl(String);

impl From<ProfileAvatarUrl> for String {
    fn from(value: ProfileAvatarUrl) -> Self {
        value.0
    }
}

impl TryFrom<String> for ProfileAvatarUrl {
    type Error = anyhow::Error;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        if value.chars().count() > 512 {
            anyhow::bail!("Invalid ProfileAvatarUrl");
        }

        Ok(Self(value))
    }
}

impl TryFrom<&str> for ProfileAvatarUrl {
    type Error = anyhow::Error;

    fn try_from(str: &str) -> Result<Self, Self::Error> {
        str.to_string().try_into()
    }
}
