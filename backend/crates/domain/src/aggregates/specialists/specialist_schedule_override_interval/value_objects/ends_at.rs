use chrono::{ NaiveTime };

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct SpecialistScheduleOverrideIntervalEndsAt(NaiveTime);

impl From<SpecialistScheduleOverrideIntervalEndsAt> for NaiveTime {
    fn from(value: SpecialistScheduleOverrideIntervalEndsAt) -> Self {
        value.0
    }
}

impl From<NaiveTime> for SpecialistScheduleOverrideIntervalEndsAt {
    fn from(value: NaiveTime) -> Self {
        Self(value)
    }
}
