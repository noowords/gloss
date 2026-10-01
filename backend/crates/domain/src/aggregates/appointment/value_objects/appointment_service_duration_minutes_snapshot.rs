use super::super::errors::AppointmentServiceError;

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct AppointmentServiceDurationMinutesSnapshot(u16);

// MARK: Conversions
impl TryFrom<u16> for AppointmentServiceDurationMinutesSnapshot {
    type Error = AppointmentServiceError;

    fn try_from(value: u16) -> Result<Self, Self::Error> {
        if value == 0 {
            return Err(AppointmentServiceError::DurationMinutesSnapshotZero);
        }

        Ok(Self(value))
    }
}

impl From<AppointmentServiceDurationMinutesSnapshot> for u16 {
    fn from(vo: AppointmentServiceDurationMinutesSnapshot) -> Self {
        vo.0
    }
}
