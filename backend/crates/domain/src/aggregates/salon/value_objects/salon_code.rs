use super::super::errors::SalonError;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct SalonCode(String);

// MARK: Conversions
impl TryFrom<String> for SalonCode {
    type Error = SalonError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        let value = value.trim();

        if value.is_empty() {
            return Err(SalonError::CodeEmpty);
        }

        if value.chars().count() > 32 {
            return Err(SalonError::CodeTooLong);
        }

        if !value.chars().all(|c| c.is_ascii_uppercase() || c.is_ascii_digit()) {
            return Err(SalonError::CodeInvalidFormat);
        }

        Ok(Self(value.to_owned()))
    }
}

impl From<SalonCode> for String {
    fn from(vo: SalonCode) -> Self {
        vo.0
    }
}
