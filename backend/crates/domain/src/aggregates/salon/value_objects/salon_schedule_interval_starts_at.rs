use chrono::NaiveTime;

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct SalonScheduleIntervalStartsAt(NaiveTime);

// MARK: Conversions
impl From<NaiveTime> for SalonScheduleIntervalStartsAt {
    fn from(value: NaiveTime) -> Self {
        Self(value)
    }
}

impl From<SalonScheduleIntervalStartsAt> for NaiveTime {
    fn from(vo: SalonScheduleIntervalStartsAt) -> Self {
        vo.0
    }
}
