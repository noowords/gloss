use chrono::NaiveDateTime;

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct OtpChallengeCreatedAt(NaiveDateTime);

// MARK: Conversions
impl From<NaiveDateTime> for OtpChallengeCreatedAt {
    fn from(value: NaiveDateTime) -> Self {
        Self(value)
    }
}

impl From<OtpChallengeCreatedAt> for NaiveDateTime {
    fn from(vo: OtpChallengeCreatedAt) -> Self {
        vo.0
    }
}
