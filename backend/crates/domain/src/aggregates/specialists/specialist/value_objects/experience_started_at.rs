use chrono::{ NaiveDate };

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct SpecialistExperienceStartedAt(NaiveDate);

impl From<SpecialistExperienceStartedAt> for NaiveDate {
    fn from(value: SpecialistExperienceStartedAt) -> Self {
        value.0
    }
}

impl From<NaiveDate> for SpecialistExperienceStartedAt {
    fn from(value: NaiveDate) -> Self {
        Self(value)
    }
}
