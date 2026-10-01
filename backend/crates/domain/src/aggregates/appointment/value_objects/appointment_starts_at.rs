use chrono::NaiveDateTime;

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct AppointmentStartsAt(NaiveDateTime);

// MARK: Conversions
impl From<NaiveDateTime> for AppointmentStartsAt {
    fn from(value: NaiveDateTime) -> Self {
        Self(value)
    }
}

impl From<AppointmentStartsAt> for NaiveDateTime {
    fn from(vo: AppointmentStartsAt) -> Self {
        vo.0
    }
}
