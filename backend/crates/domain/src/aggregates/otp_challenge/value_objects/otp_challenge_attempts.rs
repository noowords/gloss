#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct OtpChallengeAttempts(u16);

// MARK: Conversions
impl From<u16> for OtpChallengeAttempts {
    fn from(value: u16) -> Self {
        Self(value)
    }
}

impl From<OtpChallengeAttempts> for u16 {
    fn from(vo: OtpChallengeAttempts) -> Self {
        vo.0
    }
}
