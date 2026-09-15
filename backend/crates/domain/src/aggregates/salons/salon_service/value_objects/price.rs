use std::str::{ FromStr };
use bigdecimal::{ BigDecimal, Zero };

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SalonServicePrice(BigDecimal);

impl From<SalonServicePrice> for BigDecimal {
    fn from(value: SalonServicePrice) -> Self {
        value.0
    }
}

impl TryFrom<BigDecimal> for SalonServicePrice {
    type Error = anyhow::Error;

    fn try_from(value: BigDecimal) -> Result<Self, Self::Error> {
        if value < BigDecimal::zero() || value > BigDecimal::from_str("99999999.99")? || value != value.with_scale(2) {
            anyhow::bail!("Invalid SalonServicePrice");
        }

        Ok(Self(value))
    }
}

impl TryFrom<&str> for SalonServicePrice {
    type Error = anyhow::Error;

    fn try_from(str: &str) -> Result<Self, Self::Error> {
        BigDecimal::from_str(str)?.try_into()
    }
}
