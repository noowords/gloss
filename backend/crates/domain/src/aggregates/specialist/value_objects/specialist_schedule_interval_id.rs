use uuid::Uuid;

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct SpecialistScheduleIntervalId(Uuid);

// MARK: Constructors
impl SpecialistScheduleIntervalId {
    pub fn generate() -> Self {
        Self(Uuid::now_v7())
    }
}

// MARK: Conversions
impl From<Uuid> for SpecialistScheduleIntervalId {
    fn from(value: Uuid) -> Self {
        Self(value)
    }
}

impl From<SpecialistScheduleIntervalId> for Uuid {
    fn from(vo: SpecialistScheduleIntervalId) -> Self {
        vo.0
    }
}
