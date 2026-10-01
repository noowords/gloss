use super::super::errors::AppointmentError;

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct AppointmentTotalDurationMinutesSnapshot(u16);

// MARK: Conversions
impl TryFrom<u16> for AppointmentTotalDurationMinutesSnapshot {
    type Error = AppointmentError;

    fn try_from(value: u16) -> Result<Self, Self::Error> {
        if value == 0 {
            return Err(AppointmentError::TotalDurationMinutesSnapshotZero);
        }

        Ok(Self(value))
    }
}

impl From<AppointmentTotalDurationMinutesSnapshot> for u16 {
    fn from(vo: AppointmentTotalDurationMinutesSnapshot) -> Self {
        vo.0
    }
}
