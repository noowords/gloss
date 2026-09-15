use chrono::{ NaiveTime };

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct SalonScheduleIntervalStartsAt(NaiveTime);

impl From<SalonScheduleIntervalStartsAt> for NaiveTime {
    fn from(value: SalonScheduleIntervalStartsAt) -> Self {
        value.0
    }
}

impl From<NaiveTime> for SalonScheduleIntervalStartsAt {
    fn from(value: NaiveTime) -> Self {
        Self(value)
    }
}
