#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct OtpChallengeAttempts(u16);

impl From<OtpChallengeAttempts> for u16 {
    fn from(value: OtpChallengeAttempts) -> Self {
        value.0
    }
}

impl From<u16> for OtpChallengeAttempts {
    fn from(value: u16) -> Self {
        Self(value)
    }
}
