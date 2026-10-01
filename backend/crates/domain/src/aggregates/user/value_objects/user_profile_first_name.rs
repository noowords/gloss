use super::super::errors::UserProfileError;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct UserProfileFirstName(String);

// MARK: Conversions
impl TryFrom<String> for UserProfileFirstName {
    type Error = UserProfileError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        let value = value.trim();

        if value.is_empty() {
            return Err(UserProfileError::FirstNameEmpty);
        }

        if value.chars().count() > 128 {
            return Err(UserProfileError::FirstNameTooLong);
        }

        Ok(Self(value.to_owned()))
    }
}

impl From<UserProfileFirstName> for String {
    fn from(vo: UserProfileFirstName) -> Self {
        vo.0
    }
}
