use uuid::Uuid;

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct AppointmentId(Uuid);

// MARK: Constructors
impl AppointmentId {
    pub fn generate() -> Self {
        Self(Uuid::now_v7())
    }
}

// MARK: Conversions
impl From<Uuid> for AppointmentId {
    fn from(value: Uuid) -> Self {
        Self(value)
    }
}

impl From<AppointmentId> for Uuid {
    fn from(vo: AppointmentId) -> Self {
        vo.0
    }
}
