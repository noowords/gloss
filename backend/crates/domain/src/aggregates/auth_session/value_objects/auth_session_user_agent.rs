use super::super::errors::AuthSessionError;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AuthSessionUserAgent(String);

// MARK: Conversions
impl TryFrom<String> for AuthSessionUserAgent {
    type Error = AuthSessionError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        let value = value.trim();

        if value.is_empty() {
            return Err(AuthSessionError::UserAgentEmpty);
        }

        if value.chars().count() > 512 {
            return Err(AuthSessionError::UserAgentTooLong);
        }

        Ok(Self(value.to_owned()))
    }
}

impl From<AuthSessionUserAgent> for String {
    fn from(vo: AuthSessionUserAgent) -> Self {
        vo.0
    }
}
