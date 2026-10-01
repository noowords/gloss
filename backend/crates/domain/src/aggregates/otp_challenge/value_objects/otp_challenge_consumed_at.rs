use chrono::NaiveDateTime;

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct OtpChallengeConsumedAt(NaiveDateTime);

// MARK: Conversions
impl From<NaiveDateTime> for OtpChallengeConsumedAt {
    fn from(value: NaiveDateTime) -> Self {
        Self(value)
    }
}

impl From<OtpChallengeConsumedAt> for NaiveDateTime {
    fn from(vo: OtpChallengeConsumedAt) -> Self {
        vo.0
    }
}
