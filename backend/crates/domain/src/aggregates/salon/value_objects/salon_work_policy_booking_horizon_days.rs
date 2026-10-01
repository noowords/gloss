use super::super::errors::SalonWorkPolicyError;

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct SalonWorkPolicyBookingHorizonDays(u16);

// MARK: Conversions
impl TryFrom<u16> for SalonWorkPolicyBookingHorizonDays {
    type Error = SalonWorkPolicyError;

    fn try_from(value: u16) -> Result<Self, Self::Error> {
        if value == 0 {
            return Err(SalonWorkPolicyError::BookingHorizonDaysZero);
        }

        Ok(Self(value))
    }
}

impl From<SalonWorkPolicyBookingHorizonDays> for u16 {
    fn from(vo: SalonWorkPolicyBookingHorizonDays) -> Self {
        vo.0
    }
}
