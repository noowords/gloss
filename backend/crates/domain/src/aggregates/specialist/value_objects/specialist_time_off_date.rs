use chrono::NaiveDate;

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct SpecialistTimeOffDate(NaiveDate);

// MARK: Conversions
impl From<NaiveDate> for SpecialistTimeOffDate {
    fn from(value: NaiveDate) -> Self {
        Self(value)
    }
}

impl From<SpecialistTimeOffDate> for NaiveDate {
    fn from(vo: SpecialistTimeOffDate) -> Self {
        vo.0
    }
}
