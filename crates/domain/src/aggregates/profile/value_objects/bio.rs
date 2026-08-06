#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ProfileBio(String);

impl From<ProfileBio> for String {
    fn from(bio: ProfileBio) -> Self {
        bio.0
    }
}

impl From<String> for ProfileBio {
    fn from(str: String) -> Self {
        Self(str)
    }
}

impl From<&str> for ProfileBio {
    fn from(str: &str) -> Self {
        Self(str.to_string())
    }
}
