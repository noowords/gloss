use super::super::errors::SpecialistTimeOffError;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SpecialistTimeOffReason(String);

// MARK: Conversions
impl TryFrom<String> for SpecialistTimeOffReason {
    type Error = SpecialistTimeOffError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        let value = value.trim();

        if value.is_empty() {
            return Err(SpecialistTimeOffError::ReasonEmpty);
        }

        if value.chars().count() > 255 {
            return Err(SpecialistTimeOffError::ReasonTooLong);
        }

        Ok(Self(value.to_owned()))
    }
}

impl From<SpecialistTimeOffReason> for String {
    fn from(vo: SpecialistTimeOffReason) -> Self {
        vo.0
    }
}
