#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct SalonWorkPolicyDefaultAnnualLeaveMinutes(u32);

// MARK: Conversions
impl From<u32> for SalonWorkPolicyDefaultAnnualLeaveMinutes {
    fn from(value: u32) -> Self {
        Self(value)
    }
}

impl From<SalonWorkPolicyDefaultAnnualLeaveMinutes> for u32 {
    fn from(vo: SalonWorkPolicyDefaultAnnualLeaveMinutes) -> Self {
        vo.0
    }
}
