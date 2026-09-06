#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct ServiceIsActive(bool);

impl From<ServiceIsActive> for bool {
    fn from(value: ServiceIsActive) -> Self {
        value.0
    }
}

impl From<bool> for ServiceIsActive {
    fn from(value: bool) -> Self {
        ServiceIsActive(value)
    }
}
