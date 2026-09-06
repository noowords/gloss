#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ProfileAvatarUrl(String);

impl From<ProfileAvatarUrl> for String {
    fn from(avatar_url: ProfileAvatarUrl) -> Self {
        avatar_url.0
    }
}

impl From<String> for ProfileAvatarUrl {
    fn from(str: String) -> Self {
        Self(str)
    }
}

impl From<&str> for ProfileAvatarUrl {
    fn from(str: &str) -> Self {
        Self(str.to_string())
    }
}
