use chrono::{ NaiveDate };

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct AppointmentDate(NaiveDate);

impl From<AppointmentDate> for NaiveDate {
    fn from(value: AppointmentDate) -> Self {
        value.0
    }
}

impl From<NaiveDate> for AppointmentDate {
    fn from(value: NaiveDate) -> Self {
        AppointmentDate(value)
    }
}
