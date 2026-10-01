use chrono::NaiveTime;

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct SpecialistScheduleOverrideIntervalEndsAt(NaiveTime);

// MARK: Conversions
impl From<NaiveTime> for SpecialistScheduleOverrideIntervalEndsAt {
    fn from(value: NaiveTime) -> Self {
        Self(value)
    }
}

impl From<SpecialistScheduleOverrideIntervalEndsAt> for NaiveTime {
    fn from(vo: SpecialistScheduleOverrideIntervalEndsAt) -> Self {
        vo.0
    }
}
