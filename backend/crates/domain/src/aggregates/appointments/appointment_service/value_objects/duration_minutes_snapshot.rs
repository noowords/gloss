#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct AppointmentServiceDurationMinutesSnapshot(u16);

impl From<AppointmentServiceDurationMinutesSnapshot> for u16 {
    fn from(value: AppointmentServiceDurationMinutesSnapshot) -> Self {
        value.0
    }
}

impl TryFrom<u16> for AppointmentServiceDurationMinutesSnapshot {
    type Error = anyhow::Error;

    fn try_from(value: u16) -> Result<Self, Self::Error> {
        if value == 0 {
            anyhow::bail!("Invalid AppointmentServiceDurationMinutesSnapshot");
        }

        Ok(Self(value))
    }
}
