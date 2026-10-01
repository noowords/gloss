use chrono::NaiveDate;

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct SpecialistScheduleOverrideDate(NaiveDate);

// MARK: Conversions
impl From<NaiveDate> for SpecialistScheduleOverrideDate {
    fn from(value: NaiveDate) -> Self {
        Self(value)
    }
}

impl From<SpecialistScheduleOverrideDate> for NaiveDate {
    fn from(vo: SpecialistScheduleOverrideDate) -> Self {
        vo.0
    }
}
