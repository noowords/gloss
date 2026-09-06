#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ProfileFirstName(String);

impl From<ProfileFirstName> for String {
    fn from(first_name: ProfileFirstName) -> Self {
        first_name.0
    }
}

impl From<String> for ProfileFirstName {
    fn from(str: String) -> Self {
        Self(str)
    }
}

impl From<&str> for ProfileFirstName {
    fn from(str: &str) -> Self {
        Self(str.to_string())
    }
}
