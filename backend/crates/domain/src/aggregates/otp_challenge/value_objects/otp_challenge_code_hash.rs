use super::super::errors::OtpChallengeError;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OtpChallengeCodeHash(Vec<u8>);

// MARK: Conversions
impl TryFrom<Vec<u8>> for OtpChallengeCodeHash {
    type Error = OtpChallengeError;

    fn try_from(value: Vec<u8>) -> Result<Self, Self::Error> {
        if value.is_empty() {
            return Err(OtpChallengeError::CodeHashEmpty);
        }

        Ok(Self(value))
    }
}

impl From<OtpChallengeCodeHash> for Vec<u8> {
    fn from(vo: OtpChallengeCodeHash) -> Self {
        vo.0
    }
}
