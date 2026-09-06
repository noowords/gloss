use bigdecimal::{ BigDecimal };

use domain::aggregates::service::value_objects::{ ServicePrice };

#[derive(Debug, Clone, PartialEq, Eq, sqlx::Type, sqlx::FromRow)]
#[sqlx(transparent)]
pub struct MySqlServicePriceRow(BigDecimal);

impl From<MySqlServicePriceRow> for BigDecimal {
    fn from(row: MySqlServicePriceRow) -> Self {
        row.0
    }
}

impl From<BigDecimal> for MySqlServicePriceRow {
    fn from(value: BigDecimal) -> Self {
        Self(value)
    }
}

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
