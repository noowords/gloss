#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct ServiceDuration(u32);

impl From<ServiceDuration> for u32 {
    fn from(duration: ServiceDuration) -> Self {
        duration.0
    }
}

impl From<u32> for ServiceDuration {
    fn from(duration: u32) -> Self {
        Self(duration)
    }
}
