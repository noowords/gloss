use bigdecimal::BigDecimal;

use super::super::errors::AppointmentServiceError;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AppointmentServicePriceSnapshot(BigDecimal);

// MARK: Conversions
impl TryFrom<BigDecimal> for AppointmentServicePriceSnapshot {
    type Error = AppointmentServiceError;

    fn try_from(value: BigDecimal) -> Result<Self, Self::Error> {
        if value < BigDecimal::from(0) {
            return Err(AppointmentServiceError::PriceSnapshotNegative);
        }

        Ok(Self(value))
    }
}

impl From<AppointmentServicePriceSnapshot> for BigDecimal {
    fn from(vo: AppointmentServicePriceSnapshot) -> Self {
        vo.0
    }
}
