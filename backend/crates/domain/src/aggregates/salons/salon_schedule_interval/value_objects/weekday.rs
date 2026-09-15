#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct SalonScheduleIntervalWeekday(u8);

impl From<SalonScheduleIntervalWeekday> for u8 {
    fn from(value: SalonScheduleIntervalWeekday) -> Self {
        value.0
    }
}

impl TryFrom<u8> for SalonScheduleIntervalWeekday {
    type Error = anyhow::Error;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        if !(1..=7).contains(&value) {
            anyhow::bail!("Invalid SalonScheduleIntervalWeekday");
        }

        Ok(Self(value))
    }
}
