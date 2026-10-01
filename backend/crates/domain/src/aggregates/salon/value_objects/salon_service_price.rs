use bigdecimal::BigDecimal;

use super::super::errors::SalonServiceError;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SalonServicePrice(BigDecimal);

// MARK: Conversions
impl TryFrom<BigDecimal> for SalonServicePrice {
    type Error = SalonServiceError;

    fn try_from(value: BigDecimal) -> Result<Self, Self::Error> {
        if value < BigDecimal::from(0) {
            return Err(SalonServiceError::PriceNegative);
        }

        Ok(Self(value))
    }
}

impl From<SalonServicePrice> for BigDecimal {
    fn from(vo: SalonServicePrice) -> Self {
        vo.0
    }
}
