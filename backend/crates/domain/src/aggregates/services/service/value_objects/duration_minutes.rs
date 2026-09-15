#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct ServiceDurationMinutes(u16);

impl From<ServiceDurationMinutes> for u16 {
    fn from(value: ServiceDurationMinutes) -> Self {
        value.0
    }
}

impl TryFrom<u16> for ServiceDurationMinutes {
    type Error = anyhow::Error;

    fn try_from(value: u16) -> Result<Self, Self::Error> {
        if value == 0 {
            anyhow::bail!("Invalid ServiceDurationMinutes");
        }

        Ok(Self(value))
    }
}
