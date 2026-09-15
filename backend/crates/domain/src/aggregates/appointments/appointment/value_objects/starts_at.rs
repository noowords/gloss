use chrono::{ NaiveDateTime };

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct AppointmentStartsAt(NaiveDateTime);

impl From<AppointmentStartsAt> for NaiveDateTime {
    fn from(value: AppointmentStartsAt) -> Self {
        value.0
    }
}

impl From<NaiveDateTime> for AppointmentStartsAt {
    fn from(value: NaiveDateTime) -> Self {
        Self(value)
    }
}
