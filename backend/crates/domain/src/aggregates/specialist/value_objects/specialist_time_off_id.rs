use uuid::Uuid;

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct SpecialistTimeOffId(Uuid);

// MARK: Constructors
impl SpecialistTimeOffId {
    pub fn generate() -> Self {
        Self(Uuid::now_v7())
    }
}

// MARK: Conversions
impl From<Uuid> for SpecialistTimeOffId {
    fn from(value: Uuid) -> Self {
        Self(value)
    }
}

impl From<SpecialistTimeOffId> for Uuid {
    fn from(vo: SpecialistTimeOffId) -> Self {
        vo.0
    }
}
