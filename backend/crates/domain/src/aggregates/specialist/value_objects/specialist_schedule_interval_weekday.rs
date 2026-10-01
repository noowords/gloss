use chrono::Weekday;

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct SpecialistScheduleIntervalWeekday(Weekday);

// MARK: Conversions
impl From<Weekday> for SpecialistScheduleIntervalWeekday {
    fn from(value: Weekday) -> Self {
        Self(value)
    }
}

impl From<SpecialistScheduleIntervalWeekday> for Weekday {
    fn from(vo: SpecialistScheduleIntervalWeekday) -> Self {
        vo.0
    }
}
