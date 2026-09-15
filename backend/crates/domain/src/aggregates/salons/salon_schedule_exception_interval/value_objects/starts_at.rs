use chrono::{ NaiveTime };

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct SalonScheduleExceptionIntervalStartsAt(NaiveTime);

impl From<SalonScheduleExceptionIntervalStartsAt> for NaiveTime {
    fn from(value: SalonScheduleExceptionIntervalStartsAt) -> Self {
        value.0
    }
}

impl From<NaiveTime> for SalonScheduleExceptionIntervalStartsAt {
    fn from(value: NaiveTime) -> Self {
        Self(value)
    }
}
