#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct AppointmentDuration(u32);

impl From<u32> for AppointmentDuration {
    fn from(duration: u32) -> Self {
        Self(duration)
    }
}

impl From<AppointmentDuration> for u32 {
    fn from(duration: AppointmentDuration) -> Self {
        duration.0
    }
}
