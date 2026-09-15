use chrono::{ NaiveTime };

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct SpecialistScheduleOverrideIntervalStartsAt(NaiveTime);

impl From<SpecialistScheduleOverrideIntervalStartsAt> for NaiveTime {
    fn from(value: SpecialistScheduleOverrideIntervalStartsAt) -> Self {
        value.0
    }
}

impl From<NaiveTime> for SpecialistScheduleOverrideIntervalStartsAt {
    fn from(value: NaiveTime) -> Self {
        Self(value)
    }
}
