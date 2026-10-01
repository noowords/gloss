use super::super::errors::UserProfileError;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct UserProfileAvatarUrl(String);

// MARK: Conversions
impl TryFrom<String> for UserProfileAvatarUrl {
    type Error = UserProfileError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        let value = value.trim();

        if value.is_empty() {
            return Err(UserProfileError::AvatarUrlEmpty);
        }

        if value.chars().count() > 2048 {
            return Err(UserProfileError::AvatarUrlTooLong);
        }

        Ok(Self(value.to_owned()))
    }
}

impl From<UserProfileAvatarUrl> for String {
    fn from(vo: UserProfileAvatarUrl) -> Self {
        vo.0
    }
}
