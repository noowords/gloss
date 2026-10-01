use chrono::NaiveDate;

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct SpecialistExperienceStartedAt(NaiveDate);

// MARK: Conversions
impl From<NaiveDate> for SpecialistExperienceStartedAt {
    fn from(value: NaiveDate) -> Self {
        Self(value)
    }
}

impl From<SpecialistExperienceStartedAt> for NaiveDate {
    fn from(vo: SpecialistExperienceStartedAt) -> Self {
        vo.0
    }
}
