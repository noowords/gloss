use chrono::NaiveDateTime;

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct AuthSessionRevokedAt(NaiveDateTime);

// MARK: Conversions
impl From<NaiveDateTime> for AuthSessionRevokedAt {
    fn from(value: NaiveDateTime) -> Self {
        Self(value)
    }
}

impl From<AuthSessionRevokedAt> for NaiveDateTime {
    fn from(vo: AuthSessionRevokedAt) -> Self {
        vo.0
    }
}
