use chrono::{ NaiveTime };

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct SpecialistTimeOffStartsAt(NaiveTime);

impl From<SpecialistTimeOffStartsAt> for NaiveTime {
    fn from(value: SpecialistTimeOffStartsAt) -> Self {
        value.0
    }
}

impl From<NaiveTime> for SpecialistTimeOffStartsAt {
    fn from(value: NaiveTime) -> Self {
        Self(value)
    }
}
