use chrono::{ NaiveDateTime };

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct OtpChallengeExpiresAt(NaiveDateTime);

impl From<OtpChallengeExpiresAt> for NaiveDateTime {
    fn from(value: OtpChallengeExpiresAt) -> Self {
        value.0
    }
}

impl From<NaiveDateTime> for OtpChallengeExpiresAt {
    fn from(value: NaiveDateTime) -> Self {
        Self(value)
    }
}
