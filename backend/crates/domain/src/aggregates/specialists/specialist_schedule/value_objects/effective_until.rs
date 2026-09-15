use chrono::{ NaiveDate };

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct SpecialistScheduleEffectiveUntil(NaiveDate);

impl From<SpecialistScheduleEffectiveUntil> for NaiveDate {
    fn from(value: SpecialistScheduleEffectiveUntil) -> Self {
        value.0
    }
}

impl From<NaiveDate> for SpecialistScheduleEffectiveUntil {
    fn from(value: NaiveDate) -> Self {
        Self(value)
    }
}
