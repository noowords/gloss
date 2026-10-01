use chrono::Weekday;

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct SalonScheduleIntervalWeekday(Weekday);

// MARK: Conversions
impl From<Weekday> for SalonScheduleIntervalWeekday {
    fn from(value: Weekday) -> Self {
        Self(value)
    }
}

impl From<SalonScheduleIntervalWeekday> for Weekday {
    fn from(vo: SalonScheduleIntervalWeekday) -> Self {
        vo.0
    }
}
