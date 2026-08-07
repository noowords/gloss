use std::str::{ FromStr };
use bigdecimal::{ BigDecimal, Zero };

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ServicePrice(BigDecimal);

impl From<ServicePrice> for BigDecimal {
    fn from(price: ServicePrice) -> Self {
        price.0
    }
}

impl TryFrom<BigDecimal> for ServicePrice {
    type Error = anyhow::Error;

    fn try_from(value: BigDecimal) -> Result<Self, Self::Error> {
        if value < BigDecimal::zero() {
            anyhow::bail!("The price of the service cannot be negative");
        }
        
        Ok(Self(value))
    }
}

impl TryFrom<String> for ServicePrice {
    type Error = anyhow::Error;

    fn try_from(str: String) -> Result<Self, Self::Error> {
        let decimal = BigDecimal::from_str(&str)
            .map_err(|e| anyhow::anyhow!("Invalid price format: {}", e))?;
        
        decimal.try_into()
    }
}

impl TryFrom<&str> for ServicePrice {
    type Error = anyhow::Error;

    fn try_from(str: &str) -> Result<Self, Self::Error> {
        let decimal = BigDecimal::from_str(str)
            .map_err(|e| anyhow::anyhow!("Invalid price format: {}", e))?;
        
        decimal.try_into()
    }
}
