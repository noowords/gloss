use chrono::{ NaiveTime };

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct SalonScheduleExceptionIntervalEndsAt(NaiveTime);

impl From<SalonScheduleExceptionIntervalEndsAt> for NaiveTime {
    fn from(value: SalonScheduleExceptionIntervalEndsAt) -> Self {
        value.0
    }
}

impl From<NaiveTime> for SalonScheduleExceptionIntervalEndsAt {
    fn from(value: NaiveTime) -> Self {
        Self(value)
    }
}
