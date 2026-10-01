use chrono::NaiveDateTime;

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct AppointmentEndsAt(NaiveDateTime);

// MARK: Conversions
impl From<NaiveDateTime> for AppointmentEndsAt {
    fn from(value: NaiveDateTime) -> Self {
        Self(value)
    }
}

impl From<AppointmentEndsAt> for NaiveDateTime {
    fn from(vo: AppointmentEndsAt) -> Self {
        vo.0
    }
}
