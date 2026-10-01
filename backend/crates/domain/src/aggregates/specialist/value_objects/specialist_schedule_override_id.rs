use uuid::Uuid;

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct SpecialistScheduleOverrideId(Uuid);

// MARK: Constructors
impl SpecialistScheduleOverrideId {
    pub fn generate() -> Self {
        Self(Uuid::now_v7())
    }
}

// MARK: Conversions
impl From<Uuid> for SpecialistScheduleOverrideId {
    fn from(value: Uuid) -> Self {
        Self(value)
    }
}

impl From<SpecialistScheduleOverrideId> for Uuid {
    fn from(vo: SpecialistScheduleOverrideId) -> Self {
        vo.0
    }
}
