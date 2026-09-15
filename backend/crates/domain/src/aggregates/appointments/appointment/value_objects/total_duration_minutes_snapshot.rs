#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct AppointmentTotalDurationMinutesSnapshot(u16);

impl From<AppointmentTotalDurationMinutesSnapshot> for u16 {
    fn from(value: AppointmentTotalDurationMinutesSnapshot) -> Self {
        value.0
    }
}

impl TryFrom<u16> for AppointmentTotalDurationMinutesSnapshot {
    type Error = anyhow::Error;

    fn try_from(value: u16) -> Result<Self, Self::Error> {
        if value == 0 {
            anyhow::bail!("Invalid AppointmentTotalDurationMinutesSnapshot");
        }

        Ok(Self(value))
    }
}
