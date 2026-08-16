#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct SpecialistServiceIsActive(bool);

impl From<SpecialistServiceIsActive> for bool {
    fn from(value: SpecialistServiceIsActive) -> Self {
        value.0
    }
}

impl From<bool> for SpecialistServiceIsActive {
    fn from(value: bool) -> Self {
        SpecialistServiceIsActive(value)
    }
}
