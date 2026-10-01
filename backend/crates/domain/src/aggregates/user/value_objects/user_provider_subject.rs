use super::super::errors::UserProviderError;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct UserProviderSubject(String);

// MARK: Conversions
impl TryFrom<String> for UserProviderSubject {
    type Error = UserProviderError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        let value = value.trim();

        if value.is_empty() {
            return Err(UserProviderError::SubjectEmpty);
        }

        if value.chars().count() > 255 {
            return Err(UserProviderError::SubjectTooLong);
        }
        
        Ok(Self(value.to_owned()))
    }
}

impl From<UserProviderSubject> for String {
    fn from(vo: UserProviderSubject) -> Self {
        vo.0
    }
}