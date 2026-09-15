use chrono::{ NaiveTime };

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct SpecialistTimeOffEndsAt(NaiveTime);

impl From<SpecialistTimeOffEndsAt> for NaiveTime {
    fn from(value: SpecialistTimeOffEndsAt) -> Self {
        value.0
    }
}

impl From<NaiveTime> for SpecialistTimeOffEndsAt {
    fn from(value: NaiveTime) -> Self {
        Self(value)
    }
}
