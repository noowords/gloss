use chrono::{ NaiveDateTime };

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct AppointmentCancelledAt(NaiveDateTime);

impl From<AppointmentCancelledAt> for NaiveDateTime {
    fn from(value: AppointmentCancelledAt) -> Self {
        value.0
    }
}

impl From<NaiveDateTime> for AppointmentCancelledAt {
    fn from(value: NaiveDateTime) -> Self {
        Self(value)
    }
}
