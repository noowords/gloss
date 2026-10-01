use chrono::NaiveDateTime;

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct AppointmentCreatedAt(NaiveDateTime);

// MARK: Conversions
impl From<NaiveDateTime> for AppointmentCreatedAt {
    fn from(value: NaiveDateTime) -> Self {
        Self(value)
    }
}

impl From<AppointmentCreatedAt> for NaiveDateTime {
    fn from(vo: AppointmentCreatedAt) -> Self {
        vo.0
    }
}
