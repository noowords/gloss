use chrono::NaiveDateTime;

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct AuthSessionLastUsedAt(NaiveDateTime);

// MARK: Conversions
impl From<NaiveDateTime> for AuthSessionLastUsedAt {
    fn from(value: NaiveDateTime) -> Self {
        Self(value)
    }
}

impl From<AuthSessionLastUsedAt> for NaiveDateTime {
    fn from(vo: AuthSessionLastUsedAt) -> Self {
        vo.0
    }
}
