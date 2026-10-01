use chrono::NaiveTime;

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct SalonScheduleExceptionIntervalEndsAt(NaiveTime);

// MARK: Conversions
impl From<NaiveTime> for SalonScheduleExceptionIntervalEndsAt {
    fn from(value: NaiveTime) -> Self {
        Self(value)
    }
}

impl From<SalonScheduleExceptionIntervalEndsAt> for NaiveTime {
    fn from(vo: SalonScheduleExceptionIntervalEndsAt) -> Self {
        vo.0
    }
}
