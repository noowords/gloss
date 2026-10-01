use uuid::Uuid;

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct UserProviderId(Uuid);

// MARK: Constructors
impl UserProviderId {
    pub fn generate() -> Self {
        Self(Uuid::now_v7())
    }
}

// MARK: Conversions
impl From<Uuid> for UserProviderId {
    fn from(value: Uuid) -> Self {
        Self(value)
    }
}

impl From<UserProviderId> for Uuid {
    fn from(vo: UserProviderId) -> Self {
        vo.0
    }
}
