use super::super::errors::UserProfileError;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct UserProfileLastName(String);

// MARK: Conversions
impl TryFrom<String> for UserProfileLastName {
    type Error = UserProfileError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        let value = value.trim();

        if value.is_empty() {
            return Err(UserProfileError::LastNameEmpty);
        }

        if value.chars().count() > 128 {
            return Err(UserProfileError::LastNameTooLong);
        }

        Ok(Self(value.to_owned()))
    }
}

impl From<UserProfileLastName> for String {
    fn from(vo: UserProfileLastName) -> Self {
        vo.0
    }
}
