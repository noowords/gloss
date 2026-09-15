use chrono::{ NaiveDateTime };

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct AuthSessionLastUsedAt(NaiveDateTime);

impl From<AuthSessionLastUsedAt> for NaiveDateTime {
    fn from(value: AuthSessionLastUsedAt) -> Self {
        value.0
    }
}

impl From<NaiveDateTime> for AuthSessionLastUsedAt {
    fn from(value: NaiveDateTime) -> Self {
        Self(value)
    }
}
