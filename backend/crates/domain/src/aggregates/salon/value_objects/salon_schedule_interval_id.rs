use uuid::Uuid;

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct SalonScheduleIntervalId(Uuid);

// MARK: Constructors
impl SalonScheduleIntervalId {
    pub fn generate() -> Self {
        Self(Uuid::now_v7())
    }
}

// MARK: Conversions
impl From<Uuid> for SalonScheduleIntervalId {
    fn from(value: Uuid) -> Self {
        Self(value)
    }
}

impl From<SalonScheduleIntervalId> for Uuid {
    fn from(vo: SalonScheduleIntervalId) -> Self {
        vo.0
    }
}
