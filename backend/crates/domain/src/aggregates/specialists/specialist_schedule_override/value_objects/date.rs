use chrono::{ NaiveDate };

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct SpecialistScheduleOverrideDate(NaiveDate);

impl From<SpecialistScheduleOverrideDate> for NaiveDate {
    fn from(value: SpecialistScheduleOverrideDate) -> Self {
        value.0
    }
}

impl From<NaiveDate> for SpecialistScheduleOverrideDate {
    fn from(value: NaiveDate) -> Self {
        Self(value)
    }
}
