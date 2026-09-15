use chrono::{ NaiveTime };

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct SpecialistScheduleIntervalEndsAt(NaiveTime);

impl From<SpecialistScheduleIntervalEndsAt> for NaiveTime {
    fn from(value: SpecialistScheduleIntervalEndsAt) -> Self {
        value.0
    }
}

impl From<NaiveTime> for SpecialistScheduleIntervalEndsAt {
    fn from(value: NaiveTime) -> Self {
        Self(value)
    }
}
