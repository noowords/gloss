use chrono::{ NaiveDateTime };

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct OtpChallengeConsumedAt(NaiveDateTime);

impl From<OtpChallengeConsumedAt> for NaiveDateTime {
    fn from(value: OtpChallengeConsumedAt) -> Self {
        value.0
    }
}

impl From<NaiveDateTime> for OtpChallengeConsumedAt {
    fn from(value: NaiveDateTime) -> Self {
        Self(value)
    }
}
