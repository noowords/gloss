use super::super::errors::SalonWorkPolicyError;

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct SalonWorkPolicyBookingStepMinutes(u16);

// MARK: Conversions
impl TryFrom<u16> for SalonWorkPolicyBookingStepMinutes {
    type Error = SalonWorkPolicyError;

    fn try_from(value: u16) -> Result<Self, Self::Error> {
        if value == 0 {
            return Err(SalonWorkPolicyError::BookingStepMinutesZero);
        }

        Ok(Self(value))
    }
}

impl From<SalonWorkPolicyBookingStepMinutes> for u16 {
    fn from(vo: SalonWorkPolicyBookingStepMinutes) -> Self {
        vo.0
    }
}
