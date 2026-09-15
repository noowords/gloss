#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct SalonServiceIsActive(bool);

impl From<SalonServiceIsActive> for bool {
    fn from(value: SalonServiceIsActive) -> Self {
        value.0
    }
}

impl From<bool> for SalonServiceIsActive {
    fn from(value: bool) -> Self {
        Self(value)
    }
}
