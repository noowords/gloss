#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct SpecialistTimeOffChargedLeaveMinutes(u16);

impl From<SpecialistTimeOffChargedLeaveMinutes> for u16 {
    fn from(value: SpecialistTimeOffChargedLeaveMinutes) -> Self {
        value.0
    }
}

impl From<u16> for SpecialistTimeOffChargedLeaveMinutes {
    fn from(value: u16) -> Self {
        Self(value)
    }
}
