use uuid::Uuid;

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct AuthSessionId(Uuid);

// MARK: Constructors
impl AuthSessionId {
    pub fn generate() -> Self {
        Self(Uuid::now_v7())
    }
}

// MARK: Conversions
impl From<Uuid> for AuthSessionId {
    fn from(value: Uuid) -> Self {
        Self(value)
    }
}

impl From<AuthSessionId> for Uuid {
    fn from(vo: AuthSessionId) -> Self {
        vo.0
    }
}
