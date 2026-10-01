use chrono::NaiveDateTime;

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct OtpChallengeVerifiedAt(NaiveDateTime);

// MARK: Conversions
impl From<NaiveDateTime> for OtpChallengeVerifiedAt {
    fn from(value: NaiveDateTime) -> Self {
        Self(value)
    }
}

impl From<OtpChallengeVerifiedAt> for NaiveDateTime {
    fn from(vo: OtpChallengeVerifiedAt) -> Self {
        vo.0
    }
}
