#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ProfileLastName(String);

impl From<ProfileLastName> for String {
    fn from(last_name: ProfileLastName) -> Self {
        last_name.0
    }
}

impl From<String> for ProfileLastName {
    fn from(str: String) -> Self {
        Self(str)
    }
}

impl From<&str> for ProfileLastName {
    fn from(str: &str) -> Self {
        Self(str.to_string())
    }
}
