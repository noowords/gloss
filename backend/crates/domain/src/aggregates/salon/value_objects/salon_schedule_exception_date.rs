use chrono::NaiveDate;

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct SalonScheduleExceptionDate(NaiveDate);

// MARK: Conversions
impl From<NaiveDate> for SalonScheduleExceptionDate {
    fn from(value: NaiveDate) -> Self {
        Self(value)
    }
}

impl From<SalonScheduleExceptionDate> for NaiveDate {
    fn from(vo: SalonScheduleExceptionDate) -> Self {
        vo.0
    }
}
