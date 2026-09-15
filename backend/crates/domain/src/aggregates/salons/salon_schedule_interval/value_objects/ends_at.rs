use chrono::{ NaiveTime };

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct SalonScheduleIntervalEndsAt(NaiveTime);

impl From<SalonScheduleIntervalEndsAt> for NaiveTime {
    fn from(value: SalonScheduleIntervalEndsAt) -> Self {
        value.0
    }
}

impl From<NaiveTime> for SalonScheduleIntervalEndsAt {
    fn from(value: NaiveTime) -> Self {
        Self(value)
    }
}
