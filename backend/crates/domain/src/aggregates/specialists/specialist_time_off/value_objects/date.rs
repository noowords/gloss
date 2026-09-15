use chrono::{ NaiveDate };

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct SpecialistTimeOffDate(NaiveDate);

impl From<SpecialistTimeOffDate> for NaiveDate {
    fn from(value: SpecialistTimeOffDate) -> Self {
        value.0
    }
}

impl From<NaiveDate> for SpecialistTimeOffDate {
    fn from(value: NaiveDate) -> Self {
        Self(value)
    }
}
