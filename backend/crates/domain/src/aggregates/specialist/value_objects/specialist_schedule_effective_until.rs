use chrono::NaiveDate;

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct SpecialistScheduleEffectiveUntil(NaiveDate);

// MARK: Conversions
impl From<NaiveDate> for SpecialistScheduleEffectiveUntil {
    fn from(value: NaiveDate) -> Self {
        Self(value)
    }
}

impl From<SpecialistScheduleEffectiveUntil> for NaiveDate {
    fn from(vo: SpecialistScheduleEffectiveUntil) -> Self {
        vo.0
    }
}
