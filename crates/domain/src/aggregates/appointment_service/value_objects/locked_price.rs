use std::str::{ FromStr };
use bigdecimal::{ BigDecimal, Zero };

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AppointmentServiceLockedPrice(BigDecimal);

impl AppointmentServiceLockedPrice {
    pub fn new(value: BigDecimal) -> Result<Self, anyhow::Error> {
        if value < BigDecimal::zero() {
            anyhow::bail!("The price of the service cannot be negative");
        }
        
        Ok(Self(value))
    }
}

impl From<AppointmentServiceLockedPrice> for BigDecimal {
    fn from(price: AppointmentServiceLockedPrice) -> Self {
        price.0
    }
}

impl TryFrom<String> for AppointmentServiceLockedPrice {
    type Error = anyhow::Error;

    fn try_from(str: String) -> Result<Self, Self::Error> {
        let decimal = BigDecimal::from_str(&str)
            .map_err(|e| anyhow::anyhow!("Invalid price format: {}", e))?;
        
        Self::new(decimal)
    }
}

impl TryFrom<&str> for AppointmentServiceLockedPrice {
    type Error = anyhow::Error;

    fn try_from(str: &str) -> Result<Self, Self::Error> {
        let decimal = BigDecimal::from_str(str)
            .map_err(|e| anyhow::anyhow!("Invalid price format: {}", e))?;
        
        Self::new(decimal)
    }
}
