use uuid::Uuid;

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct SalonId(Uuid);

// MARK: Constructors
impl SalonId {
    pub fn generate() -> Self {
        Self(Uuid::now_v7())
    }
}

// MARK: Conversions
impl From<Uuid> for SalonId {
    fn from(value: Uuid) -> Self {
        Self(value)
    }
}

impl From<SalonId> for Uuid {
    fn from(vo: SalonId) -> Self {
        vo.0
    }
}
