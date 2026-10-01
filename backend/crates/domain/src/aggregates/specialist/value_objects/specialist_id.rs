use uuid::Uuid;

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct SpecialistId(Uuid);

// MARK: Constructors
impl SpecialistId {
    pub fn generate() -> Self {
        Self(Uuid::now_v7())
    }
}

// MARK: Conversions
impl From<Uuid> for SpecialistId {
    fn from(value: Uuid) -> Self {
        Self(value)
    }
}

impl From<SpecialistId> for Uuid {
    fn from(vo: SpecialistId) -> Self {
        vo.0
    }
}
