use super::super::errors::SalonError;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SalonName(String);

// MARK: Conversions
impl TryFrom<String> for SalonName {
    type Error = SalonError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        let value = value.trim();

        if value.is_empty() {
            return Err(SalonError::NameEmpty);
        }

        if value.chars().count() > 128 {
            return Err(SalonError::NameTooLong);
        }

        Ok(Self(value.to_owned()))
    }
}

impl From<SalonName> for String {
    fn from(vo: SalonName) -> Self {
        vo.0
    }
}
