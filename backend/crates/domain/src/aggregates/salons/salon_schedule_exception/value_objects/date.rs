use chrono::{ NaiveDate };

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct SalonScheduleExceptionDate(NaiveDate);

impl From<SalonScheduleExceptionDate> for NaiveDate {
    fn from(value: SalonScheduleExceptionDate) -> Self {
        value.0
    }
}

impl From<NaiveDate> for SalonScheduleExceptionDate {
    fn from(value: NaiveDate) -> Self {
        Self(value)
    }
}
