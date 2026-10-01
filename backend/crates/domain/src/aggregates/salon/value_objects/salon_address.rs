use super::super::errors::SalonError;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SalonAddress(String);

// MARK: Conversions
impl TryFrom<String> for SalonAddress {
    type Error = SalonError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        let value = value.trim();

        if value.is_empty() {
            return Err(SalonError::AddressEmpty);
        }

        if value.chars().count() > 255 {
            return Err(SalonError::AddressTooLong);
        }

        Ok(Self(value.to_owned()))
    }
}

impl From<SalonAddress> for String {
    fn from(vo: SalonAddress) -> Self {
        vo.0
    }
}
