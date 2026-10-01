use chrono::NaiveTime;

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct SpecialistTimeOffEndsAt(NaiveTime);

// MARK: Conversions
impl From<NaiveTime> for SpecialistTimeOffEndsAt {
    fn from(value: NaiveTime) -> Self {
        Self(value)
    }
}

impl From<SpecialistTimeOffEndsAt> for NaiveTime {
    fn from(vo: SpecialistTimeOffEndsAt) -> Self {
        vo.0
    }
}
