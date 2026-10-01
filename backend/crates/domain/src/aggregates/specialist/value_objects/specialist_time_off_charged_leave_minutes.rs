#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct SpecialistTimeOffChargedLeaveMinutes(u16);

// MARK: Conversions
impl From<u16> for SpecialistTimeOffChargedLeaveMinutes {
    fn from(value: u16) -> Self {
        Self(value)
    }
}

impl From<SpecialistTimeOffChargedLeaveMinutes> for u16 {
    fn from(vo: SpecialistTimeOffChargedLeaveMinutes) -> Self {
        vo.0
    }
}
