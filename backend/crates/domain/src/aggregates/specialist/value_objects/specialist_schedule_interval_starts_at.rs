use chrono::NaiveTime;

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct SpecialistScheduleIntervalStartsAt(NaiveTime);

// MARK: Conversions
impl From<NaiveTime> for SpecialistScheduleIntervalStartsAt {
    fn from(value: NaiveTime) -> Self {
        Self(value)
    }
}

impl From<SpecialistScheduleIntervalStartsAt> for NaiveTime {
    fn from(vo: SpecialistScheduleIntervalStartsAt) -> Self {
        vo.0
    }
}
