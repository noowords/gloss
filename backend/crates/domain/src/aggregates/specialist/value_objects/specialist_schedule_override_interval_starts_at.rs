use chrono::NaiveTime;

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct SpecialistScheduleOverrideIntervalStartsAt(NaiveTime);

// MARK: Conversions
impl From<NaiveTime> for SpecialistScheduleOverrideIntervalStartsAt {
    fn from(value: NaiveTime) -> Self {
        Self(value)
    }
}

impl From<SpecialistScheduleOverrideIntervalStartsAt> for NaiveTime {
    fn from(vo: SpecialistScheduleOverrideIntervalStartsAt) -> Self {
        vo.0
    }
}