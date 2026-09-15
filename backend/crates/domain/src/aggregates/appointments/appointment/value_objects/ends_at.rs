use chrono::{ NaiveDateTime };

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct AppointmentEndsAt(NaiveDateTime);

impl From<AppointmentEndsAt> for NaiveDateTime {
    fn from(value: AppointmentEndsAt) -> Self {
        value.0
    }
}

impl From<NaiveDateTime> for AppointmentEndsAt {
    fn from(value: NaiveDateTime) -> Self {
        Self(value)
    }
}
