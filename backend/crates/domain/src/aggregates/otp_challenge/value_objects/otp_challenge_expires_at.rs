use chrono::NaiveDateTime;

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct OtpChallengeExpiresAt(NaiveDateTime);

// MARK: Conversions
impl From<NaiveDateTime> for OtpChallengeExpiresAt {
    fn from(value: NaiveDateTime) -> Self {
        Self(value)
    }
}

impl From<OtpChallengeExpiresAt> for NaiveDateTime {
    fn from(vo: OtpChallengeExpiresAt) -> Self {
        vo.0
    }
}
