use chrono::NaiveTime;

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct SalonScheduleExceptionIntervalStartsAt(NaiveTime);

// MARK: Conversions
impl From<NaiveTime> for SalonScheduleExceptionIntervalStartsAt {
    fn from(value: NaiveTime) -> Self {
        Self(value)
    }
}

impl From<SalonScheduleExceptionIntervalStartsAt> for NaiveTime {
    fn from(vo: SalonScheduleExceptionIntervalStartsAt) -> Self {
        vo.0
    }
}
