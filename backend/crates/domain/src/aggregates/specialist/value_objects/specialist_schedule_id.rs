use uuid::Uuid;

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct SpecialistScheduleId(Uuid);

// MARK: Constructors
impl SpecialistScheduleId {
    pub fn generate() -> Self {
        Self(Uuid::now_v7())
    }
}

// MARK: Conversions
impl From<Uuid> for SpecialistScheduleId {
    fn from(value: Uuid) -> Self {
        Self(value)
    }
}

impl From<SpecialistScheduleId> for Uuid {
    fn from(vo: SpecialistScheduleId) -> Self {
        vo.0
    }
}
