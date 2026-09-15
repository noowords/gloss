#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct SpecialistLeaveAllowanceYear(u16);

impl From<SpecialistLeaveAllowanceYear> for u16 {
    fn from(value: SpecialistLeaveAllowanceYear) -> Self {
        value.0
    }
}

impl From<u16> for SpecialistLeaveAllowanceYear {
    fn from(value: u16) -> Self {
        Self(value)
    }
}
