#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct SalonWorkPolicyBookingStepMinutes(u16);

impl From<SalonWorkPolicyBookingStepMinutes> for u16 {
    fn from(value: SalonWorkPolicyBookingStepMinutes) -> Self {
        value.0
    }
}

impl TryFrom<u16> for SalonWorkPolicyBookingStepMinutes {
    type Error = anyhow::Error;

    fn try_from(value: u16) -> Result<Self, Self::Error> {
        if value == 0 {
            anyhow::bail!("Invalid SalonWorkPolicyBookingStepMinutes");
        }

        Ok(Self(value))
    }
}
