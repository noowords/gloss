use bigdecimal::BigDecimal;

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
