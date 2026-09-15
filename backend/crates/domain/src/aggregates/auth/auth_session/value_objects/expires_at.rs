use chrono::{ NaiveDateTime };

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct AuthSessionExpiresAt(NaiveDateTime);

impl From<AuthSessionExpiresAt> for NaiveDateTime {
    fn from(value: AuthSessionExpiresAt) -> Self {
        value.0
    }
}

impl From<NaiveDateTime> for AuthSessionExpiresAt {
    fn from(value: NaiveDateTime) -> Self {
        Self(value)
    }
}
