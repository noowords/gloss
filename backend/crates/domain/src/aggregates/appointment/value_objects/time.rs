use chrono::{ NaiveTime };

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct AppointmentTime(NaiveTime);

impl From<AppointmentTime> for NaiveTime {
    fn from(value: AppointmentTime) -> Self {
        value.0
    }
}

impl From<NaiveTime> for AppointmentTime {
    fn from(value: NaiveTime) -> Self {
        AppointmentTime(value)
    }
}
