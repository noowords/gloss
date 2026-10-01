use uuid::Uuid;

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct AppointmentReviewId(Uuid);

// MARK: Constructors
impl AppointmentReviewId {
    pub fn generate() -> Self {
        Self(Uuid::now_v7())
    }
}

// MARK: Conversions
impl From<Uuid> for AppointmentReviewId {
    fn from(value: Uuid) -> Self {
        Self(value)
    }
}

impl From<AppointmentReviewId> for Uuid {
    fn from(vo: AppointmentReviewId) -> Self {
        vo.0
    }
}
