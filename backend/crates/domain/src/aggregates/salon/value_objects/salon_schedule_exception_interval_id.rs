use uuid::Uuid;

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct SalonScheduleExceptionIntervalId(Uuid);

// MARK: Constructors
impl SalonScheduleExceptionIntervalId {
    pub fn generate() -> Self {
        Self(Uuid::now_v7())
    }
}

// MARK: Conversions
impl From<Uuid> for SalonScheduleExceptionIntervalId {
    fn from(value: Uuid) -> Self {
        Self(value)
    }
}

impl From<SalonScheduleExceptionIntervalId> for Uuid {
    fn from(vo: SalonScheduleExceptionIntervalId) -> Self {
        vo.0
    }
}
