use domain::aggregates::salons::salon::Salon;

use super::value_objects::{
    MySqlSalonAddressRow,
    MySqlSalonCityRow,
    MySqlSalonCodeRow,
    MySqlSalonIdRow,
    MySqlSalonNameRow,
    MySqlSalonStatusRow,
    MySqlSalonTimezoneRow
};

#[derive(Debug, Clone, sqlx::FromRow)]
pub struct MySqlSalonRow {
    pub id: MySqlSalonIdRow,
    pub code: MySqlSalonCodeRow,
    pub name: MySqlSalonNameRow,
    pub city: MySqlSalonCityRow,
    pub address: Option<MySqlSalonAddressRow>,
    pub timezone: MySqlSalonTimezoneRow,
    pub status: MySqlSalonStatusRow
}

impl TryFrom<MySqlSalonRow> for Salon {
    type Error = anyhow::Error;

    fn try_from(row: MySqlSalonRow) -> Result<Self, Self::Error> {
        Salon::restore(
            uuid::Uuid::from(row.id).into(),
            String::from(row.code).try_into()?,
            String::from(row.name).try_into()?,
            String::from(row.city).try_into()?,
            row.address.map(|value| String::from(value).try_into()).transpose()?,
            String::from(row.timezone).try_into()?,
            String::from(row.status).try_into()?
        )
    }
}

impl From<&Salon> for MySqlSalonRow {
    fn from(entity: &Salon) -> Self {
        Self {
            id: uuid::Uuid::from(entity.id()).into(),
            code: String::from(entity.code()).into(),
            name: String::from(entity.name()).into(),
            city: String::from(entity.city()).into(),
            address: entity.address().map(|value| String::from(value).into()),
            timezone: String::from(entity.timezone()).into(),
            status: String::from(entity.status()).into()
        }
    }
}
