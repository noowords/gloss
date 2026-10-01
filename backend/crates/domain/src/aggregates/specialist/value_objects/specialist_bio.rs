use super::super::errors::SpecialistError;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SpecialistBio(String);

// MARK: Conversions
impl TryFrom<String> for SpecialistBio {
    type Error = SpecialistError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        let value = value.trim();

        if value.is_empty() {
            return Err(SpecialistError::BioEmpty);
        }

        if value.chars().count() > 1024 {
            return Err(SpecialistError::BioTooLong);
        }

        Ok(Self(value.to_owned()))
    }
}

impl From<SpecialistBio> for String {
    fn from(vo: SpecialistBio) -> Self {
        vo.0
    }
}
