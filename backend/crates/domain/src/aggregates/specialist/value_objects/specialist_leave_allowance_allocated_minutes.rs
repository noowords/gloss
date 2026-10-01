#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct SpecialistLeaveAllowanceAllocatedMinutes(u32);

// MARK: Conversions
impl From<u32> for SpecialistLeaveAllowanceAllocatedMinutes {
    fn from(value: u32) -> Self {
        Self(value)
    }
}

impl From<SpecialistLeaveAllowanceAllocatedMinutes> for u32 {
    fn from(vo: SpecialistLeaveAllowanceAllocatedMinutes) -> Self {
        vo.0
    }
}
