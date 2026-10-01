use super::super::errors::OtpChallengeError;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct OtpChallengeSubject(String);

// MARK: Conversions
impl TryFrom<String> for OtpChallengeSubject {
    type Error = OtpChallengeError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        let value = value.trim();

        if value.is_empty() {
            return Err(OtpChallengeError::SubjectEmpty);
        }

        if value.chars().count() > 255 {
            return Err(OtpChallengeError::SubjectTooLong);
        }

        Ok(Self(value.to_owned()))
    }
}

impl From<OtpChallengeSubject> for String {
    fn from(vo: OtpChallengeSubject) -> Self {
        vo.0
    }
}
