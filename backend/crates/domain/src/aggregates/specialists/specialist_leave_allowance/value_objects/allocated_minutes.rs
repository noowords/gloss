#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct SpecialistLeaveAllowanceAllocatedMinutes(u32);

impl From<SpecialistLeaveAllowanceAllocatedMinutes> for u32 {
    fn from(value: SpecialistLeaveAllowanceAllocatedMinutes) -> Self {
        value.0
    }
}

impl From<u32> for SpecialistLeaveAllowanceAllocatedMinutes {
    fn from(value: u32) -> Self {
        Self(value)
    }
}
