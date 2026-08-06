use std::str::{ FromStr };
use bigdecimal::{ BigDecimal, Zero };

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ServicePrice(BigDecimal);

impl ServicePrice {
    pub fn new(value: BigDecimal) -> Result<Self, anyhow::Error> {
        if value < BigDecimal::zero() {
            anyhow::bail!("The price of the service cannot be negative");
        }
        
        Ok(Self(value))
    }
}

impl From<ServicePrice> for BigDecimal {
    fn from(price: ServicePrice) -> Self {
        price.0
    }
}

impl TryFrom<String> for ServicePrice {
    type Error = anyhow::Error;

    fn try_from(str: String) -> Result<Self, Self::Error> {
        let decimal = BigDecimal::from_str(&str)
            .map_err(|e| anyhow::anyhow!("Invalid price format: {}", e))?;
        
        Self::new(decimal)
    }
}

impl TryFrom<&str> for ServicePrice {
    type Error = anyhow::Error;

    fn try_from(str: &str) -> Result<Self, Self::Error> {
        let decimal = BigDecimal::from_str(str)
            .map_err(|e| anyhow::anyhow!("Invalid price format: {}", e))?;
        
        Self::new(decimal)
    }
}
