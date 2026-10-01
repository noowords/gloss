use chrono::NaiveTime;

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct SpecialistTimeOffStartsAt(NaiveTime);

// MARK: Conversions
impl From<NaiveTime> for SpecialistTimeOffStartsAt {
    fn from(value: NaiveTime) -> Self {
        Self(value)
    }
}

impl From<SpecialistTimeOffStartsAt> for NaiveTime {
    fn from(vo: SpecialistTimeOffStartsAt) -> Self {
        vo.0
    }
}
