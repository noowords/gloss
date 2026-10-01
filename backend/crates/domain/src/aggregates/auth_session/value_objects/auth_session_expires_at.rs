use chrono::NaiveDateTime;

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct AuthSessionExpiresAt(NaiveDateTime);

// MARK: Conversions
impl From<NaiveDateTime> for AuthSessionExpiresAt {
    fn from(value: NaiveDateTime) -> Self {
        Self(value)
    }
}

impl From<AuthSessionExpiresAt> for NaiveDateTime {
    fn from(vo: AuthSessionExpiresAt) -> Self {
        vo.0
    }
}
