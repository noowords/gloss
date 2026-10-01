use chrono::NaiveDate;

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct SpecialistScheduleEffectiveFrom(NaiveDate);

// MARK: Conversions
impl From<NaiveDate> for SpecialistScheduleEffectiveFrom {
    fn from(value: NaiveDate) -> Self {
        Self(value)
    }
}

impl From<SpecialistScheduleEffectiveFrom> for NaiveDate {
    fn from(vo: SpecialistScheduleEffectiveFrom) -> Self {
        vo.0
    }
}
