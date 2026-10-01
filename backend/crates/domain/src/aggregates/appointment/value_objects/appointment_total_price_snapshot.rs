use bigdecimal::BigDecimal;

use super::super::errors::AppointmentError;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AppointmentTotalPriceSnapshot(BigDecimal);

// MARK: Conversions
impl TryFrom<BigDecimal> for AppointmentTotalPriceSnapshot {
    type Error = AppointmentError;

    fn try_from(value: BigDecimal) -> Result<Self, Self::Error> {
        if value < BigDecimal::from(0) {
            return Err(AppointmentError::TotalPriceSnapshotNegative);
        }

        Ok(Self(value))
    }
}

impl From<AppointmentTotalPriceSnapshot> for BigDecimal {
    fn from(vo: AppointmentTotalPriceSnapshot) -> Self {
        vo.0
    }
}
