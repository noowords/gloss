#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct SpecialistLeaveAllowanceYear(u16);

// MARK: Conversions
impl From<u16> for SpecialistLeaveAllowanceYear {
    fn from(value: u16) -> Self {
        Self(value)
    }
}

impl From<SpecialistLeaveAllowanceYear> for u16 {
    fn from(vo: SpecialistLeaveAllowanceYear) -> Self {
        vo.0
    }
}
