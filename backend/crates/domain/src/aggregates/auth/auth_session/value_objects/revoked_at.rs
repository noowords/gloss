use chrono::{ NaiveDateTime };

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct AuthSessionRevokedAt(NaiveDateTime);

impl From<AuthSessionRevokedAt> for NaiveDateTime {
    fn from(value: AuthSessionRevokedAt) -> Self {
        value.0
    }
}

impl From<NaiveDateTime> for AuthSessionRevokedAt {
    fn from(value: NaiveDateTime) -> Self {
        Self(value)
    }
}
