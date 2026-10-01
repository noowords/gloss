use chrono::NaiveTime;

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct SalonScheduleIntervalEndsAt(NaiveTime);

// MARK: Conversions
impl From<NaiveTime> for SalonScheduleIntervalEndsAt {
    fn from(value: NaiveTime) -> Self {
        Self(value)
    }
}

impl From<SalonScheduleIntervalEndsAt> for NaiveTime {
    fn from(vo: SalonScheduleIntervalEndsAt) -> Self {
        vo.0
    }
}
