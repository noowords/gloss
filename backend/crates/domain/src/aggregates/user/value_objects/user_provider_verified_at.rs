use chrono::NaiveDateTime;

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct UserProviderVerifiedAt(NaiveDateTime);

// MARK: Conversions
impl From<NaiveDateTime> for UserProviderVerifiedAt {
    fn from(value: NaiveDateTime) -> Self {
        Self(value)
    }
}

impl From<UserProviderVerifiedAt> for NaiveDateTime {
    fn from(vo: UserProviderVerifiedAt) -> Self {
        vo.0
    }
}
