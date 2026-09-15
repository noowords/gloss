#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct SpecialistScheduleIntervalWeekday(u8);

impl From<SpecialistScheduleIntervalWeekday> for u8 {
    fn from(value: SpecialistScheduleIntervalWeekday) -> Self {
        value.0
    }
}

impl TryFrom<u8> for SpecialistScheduleIntervalWeekday {
    type Error = anyhow::Error;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        if !(1..=7).contains(&value) {
            anyhow::bail!("Invalid SpecialistScheduleIntervalWeekday");
        }

        Ok(Self(value))
    }
}
