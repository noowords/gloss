use super::super::errors::ServiceError;

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct ServiceDurationMinutes(u16);

// MARK: Conversions
impl TryFrom<u16> for ServiceDurationMinutes {
    type Error = ServiceError;

    fn try_from(value: u16) -> Result<Self, Self::Error> {
        if value == 0 {
            return Err(ServiceError::DurationMinutesZero);
        }

        Ok(Self(value))
    }
}

impl From<ServiceDurationMinutes> for u16 {
    fn from(vo: ServiceDurationMinutes) -> Self {
        vo.0
    }
}