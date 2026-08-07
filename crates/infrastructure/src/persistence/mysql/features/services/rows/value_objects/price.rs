use bigdecimal::{ BigDecimal };

use domain::aggregates::service::value_objects::{ ServicePrice };

#[derive(Debug, Clone, PartialEq, Eq, sqlx::Type)]
#[sqlx(transparent)]
pub struct MySqlServicePriceRow(BigDecimal);

impl TryFrom<MySqlServicePriceRow> for ServicePrice {
    type Error = anyhow::Error;
    
    fn try_from(row: MySqlServicePriceRow) -> Result<Self, Self::Error> {
        row.0.try_into()
    }
}

impl From<ServicePrice> for MySqlServicePriceRow {
    fn from(entity: ServicePrice) -> Self {
        Self(entity.into())
    }
}
