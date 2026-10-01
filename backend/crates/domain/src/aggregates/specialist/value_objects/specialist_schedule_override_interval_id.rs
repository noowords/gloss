use uuid::Uuid;

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct SpecialistScheduleOverrideIntervalId(Uuid);

// MARK: Constructors
impl SpecialistScheduleOverrideIntervalId {
    pub fn generate() -> Self {
        Self(Uuid::now_v7())
    }
}

// MARK: Conversions
impl From<Uuid> for SpecialistScheduleOverrideIntervalId {
    fn from(value: Uuid) -> Self {
        Self(value)
    }
}

impl From<SpecialistScheduleOverrideIntervalId> for Uuid {
    fn from(vo: SpecialistScheduleOverrideIntervalId) -> Self {
        vo.0
    }
}
