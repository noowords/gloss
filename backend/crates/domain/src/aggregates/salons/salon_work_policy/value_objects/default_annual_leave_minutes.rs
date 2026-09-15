#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct SalonWorkPolicyDefaultAnnualLeaveMinutes(u32);

impl From<SalonWorkPolicyDefaultAnnualLeaveMinutes> for u32 {
    fn from(value: SalonWorkPolicyDefaultAnnualLeaveMinutes) -> Self {
        value.0
    }
}

impl From<u32> for SalonWorkPolicyDefaultAnnualLeaveMinutes {
    fn from(value: u32) -> Self {
        Self(value)
    }
}
