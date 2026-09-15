use chrono::{ NaiveDate };

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct SpecialistScheduleEffectiveFrom(NaiveDate);

impl From<SpecialistScheduleEffectiveFrom> for NaiveDate {
    fn from(value: SpecialistScheduleEffectiveFrom) -> Self {
        value.0
    }
}

impl From<NaiveDate> for SpecialistScheduleEffectiveFrom {
    fn from(value: NaiveDate) -> Self {
        Self(value)
    }
}
