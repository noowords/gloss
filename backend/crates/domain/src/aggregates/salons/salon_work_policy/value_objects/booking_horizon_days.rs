#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct SalonWorkPolicyBookingHorizonDays(u16);

impl From<SalonWorkPolicyBookingHorizonDays> for u16 {
    fn from(value: SalonWorkPolicyBookingHorizonDays) -> Self {
        value.0
    }
}

impl TryFrom<u16> for SalonWorkPolicyBookingHorizonDays {
    type Error = anyhow::Error;

    fn try_from(value: u16) -> Result<Self, Self::Error> {
        if value == 0 {
            anyhow::bail!("Invalid SalonWorkPolicyBookingHorizonDays");
        }

        Ok(Self(value))
    }
}
