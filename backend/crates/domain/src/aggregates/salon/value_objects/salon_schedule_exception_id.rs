use uuid::Uuid;

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct SalonScheduleExceptionId(Uuid);

// MARK: Constructors
impl SalonScheduleExceptionId {
    pub fn generate() -> Self {
        Self(Uuid::now_v7())
    }
}

// MARK: Conversions
impl From<Uuid> for SalonScheduleExceptionId {
    fn from(value: Uuid) -> Self {
        Self(value)
    }
}

impl From<SalonScheduleExceptionId> for Uuid {
    fn from(vo: SalonScheduleExceptionId) -> Self {
        vo.0
    }
}
