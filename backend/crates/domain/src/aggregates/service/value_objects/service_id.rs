use uuid::Uuid;

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct ServiceId(Uuid);

// MARK: Constructors
impl ServiceId {
    pub fn generate() -> Self {
        Self(Uuid::now_v7())
    }
}

// MARK: Conversions
impl From<Uuid> for ServiceId {
    fn from(value: Uuid) -> Self {
        Self(value)
    }
}

impl From<ServiceId> for Uuid {
    fn from(vo: ServiceId) -> Self {
        vo.0
    }
}
