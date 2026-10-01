use chrono::NaiveDateTime;

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct AuthSessionCreatedAt(NaiveDateTime);

// MARK: Conversions
impl From<NaiveDateTime> for AuthSessionCreatedAt {
    fn from(value: NaiveDateTime) -> Self {
        Self(value)
    }
}

impl From<AuthSessionCreatedAt> for NaiveDateTime {
    fn from(vo: AuthSessionCreatedAt) -> Self {
        vo.0
    }
}
