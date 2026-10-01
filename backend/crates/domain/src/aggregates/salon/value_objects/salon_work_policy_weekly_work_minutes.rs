use super::super::errors::SalonWorkPolicyError;

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct SalonWorkPolicyWeeklyWorkMinutes(u16);

// MARK: Conversions
impl TryFrom<u16> for SalonWorkPolicyWeeklyWorkMinutes {
    type Error = SalonWorkPolicyError;

    fn try_from(value: u16) -> Result<Self, Self::Error> {
        if value == 0 {
            return Err(SalonWorkPolicyError::WeeklyWorkMinutesZero);
        }

        Ok(Self(value))
    }
}

impl From<SalonWorkPolicyWeeklyWorkMinutes> for u16 {
    fn from(vo: SalonWorkPolicyWeeklyWorkMinutes) -> Self {
        vo.0
    }
}
