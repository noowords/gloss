use chrono::{ NaiveDateTime };

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct UserProviderVerifiedAt(NaiveDateTime);

impl From<UserProviderVerifiedAt> for NaiveDateTime {
    fn from(value: UserProviderVerifiedAt) -> Self {
        value.0
    }
}

impl From<NaiveDateTime> for UserProviderVerifiedAt {
    fn from(value: NaiveDateTime) -> Self {
        Self(value)
    }
}
