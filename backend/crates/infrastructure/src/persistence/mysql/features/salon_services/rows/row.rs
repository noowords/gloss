use domain::aggregates::salons::salon_service::SalonService;

use super::value_objects::{
    MySqlSalonServiceIsActiveRow,
    MySqlSalonServicePriceRow,
    MySqlSalonServiceSalonIdRow,
    MySqlSalonServiceServiceIdRow
};

#[derive(Debug, Clone, sqlx::FromRow)]
pub struct MySqlSalonServiceRow {
    pub salon_id: MySqlSalonServiceSalonIdRow,
    pub service_id: MySqlSalonServiceServiceIdRow,
    pub price: MySqlSalonServicePriceRow,
    pub is_active: MySqlSalonServiceIsActiveRow
}

impl TryFrom<MySqlSalonServiceRow> for SalonService {
    type Error = anyhow::Error;

    fn try_from(row: MySqlSalonServiceRow) -> Result<Self, Self::Error> {
        SalonService::restore(
            uuid::Uuid::from(row.salon_id).into(),
            uuid::Uuid::from(row.service_id).into(),
            bigdecimal::BigDecimal::from(row.price).try_into()?,
            bool::from(row.is_active).into()
        )
    }
}

impl From<&SalonService> for MySqlSalonServiceRow {
    fn from(entity: &SalonService) -> Self {
        Self {
            salon_id: uuid::Uuid::from(entity.salon_id()).into(),
            service_id: uuid::Uuid::from(entity.service_id()).into(),
            price: bigdecimal::BigDecimal::from(entity.price()).into(),
            is_active: bool::from(entity.is_active()).into()
        }
    }
}
