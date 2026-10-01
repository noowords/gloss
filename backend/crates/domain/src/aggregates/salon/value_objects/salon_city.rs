use super::super::errors::SalonError;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SalonCity(String);

// MARK: Conversions
impl TryFrom<String> for SalonCity {
    type Error = SalonError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        let value = value.trim();

        if value.is_empty() {
            return Err(SalonError::CityEmpty);
        }

        if value.chars().count() > 128 {
            return Err(SalonError::CityTooLong);
        }

        Ok(Self(value.to_owned()))
    }
}

impl From<SalonCity> for String {
    fn from(vo: SalonCity) -> Self {
        vo.0
    }
}
