use chrono::{ NaiveDateTime };

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct OtpChallengeVerifiedAt(NaiveDateTime);

impl From<OtpChallengeVerifiedAt> for NaiveDateTime {
    fn from(value: OtpChallengeVerifiedAt) -> Self {
        value.0
    }
}

impl From<NaiveDateTime> for OtpChallengeVerifiedAt {
    fn from(value: NaiveDateTime) -> Self {
        Self(value)
    }
}
