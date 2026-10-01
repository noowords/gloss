use chrono::NaiveDateTime;

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct AppointmentCancelledAt(NaiveDateTime);

// MARK: Conversions
impl From<NaiveDateTime> for AppointmentCancelledAt {
    fn from(value: NaiveDateTime) -> Self {
        Self(value)
    }
}

impl From<AppointmentCancelledAt> for NaiveDateTime {
    fn from(vo: AppointmentCancelledAt) -> Self {
        vo.0
    }
}
