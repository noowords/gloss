use chrono::{ NaiveTime };

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct SpecialistScheduleIntervalStartsAt(NaiveTime);

impl From<SpecialistScheduleIntervalStartsAt> for NaiveTime {
    fn from(value: SpecialistScheduleIntervalStartsAt) -> Self {
        value.0
    }
}

impl From<NaiveTime> for SpecialistScheduleIntervalStartsAt {
    fn from(value: NaiveTime) -> Self {
        Self(value)
    }
}
