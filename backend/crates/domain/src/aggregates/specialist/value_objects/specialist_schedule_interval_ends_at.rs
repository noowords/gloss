use chrono::NaiveTime;

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct SpecialistScheduleIntervalEndsAt(NaiveTime);

// MARK: Conversions
impl From<NaiveTime> for SpecialistScheduleIntervalEndsAt {
    fn from(value: NaiveTime) -> Self {
        Self(value)
    }
}

impl From<SpecialistScheduleIntervalEndsAt> for NaiveTime {
    fn from(vo: SpecialistScheduleIntervalEndsAt) -> Self {
        vo.0
    }
}
