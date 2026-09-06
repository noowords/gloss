use chrono::{ DateTime, Utc, Duration };

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct OtpExpiresAt(DateTime<Utc>);

impl OtpExpiresAt {
    pub fn is_expired(&self) -> bool {
        self.0 < Utc::now()
    }

    pub fn in_minutes(minutes: i64) -> Self {
        Self(Utc::now() + Duration::minutes(minutes))
    }
}

impl From<OtpExpiresAt> for DateTime<Utc> {
    fn from(expires_at: OtpExpiresAt) -> Self {
        expires_at.0
    }
}

impl From<DateTime<Utc>> for OtpExpiresAt {
    fn from(expires_at: DateTime<Utc>) -> Self {
        Self(expires_at)
    }
}
