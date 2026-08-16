use domain::aggregates::service::{ Service };

use super::value_objects::{ MySqlServiceIdRow, MySqlServiceCategoryRow, MySqlServiceNameRow, MySqlServiceDescriptionRow, MySqlServiceCoverUrlRow, MySqlServicePriceRow, MySqlServiceDurationRow, MySqlServiceIsActiveRow };

#[derive(Debug, Clone, sqlx::FromRow)]
pub struct MySqlServiceRow {
    pub id: MySqlServiceIdRow,
    pub category: MySqlServiceCategoryRow,
    pub name: MySqlServiceNameRow,
    pub description: Option<MySqlServiceDescriptionRow>,
    pub cover_url: Option<MySqlServiceCoverUrlRow>,
    pub price: MySqlServicePriceRow,
    pub duration: MySqlServiceDurationRow,
    pub is_active: MySqlServiceIsActiveRow
}

impl TryFrom<MySqlServiceRow> for Service {
    type Error = anyhow::Error;
    
    fn try_from(row: MySqlServiceRow) -> Result<Self, Self::Error> {
        Ok(Self::restore(
            row.id.into(),
            row.category.try_into()?,
            row.name.into(),
            row.description.map(|d| d.into()),
            row.cover_url.map(|cu| cu.into()),
            row.price.try_into()?,
            row.duration.into(),
            row.is_active.into()
        ))
    }
}

impl From<&Service> for MySqlServiceRow {
    fn from(entity: &Service) -> Self {
        Self {
            id: entity.id().into(),
            category: entity.category().into(),
            name: entity.name().into(),
            description: entity.description().map(|d| d.into()),
            cover_url: entity.cover_url().map(|cu| cu.into()),
            price: entity.price().into(),
            duration: entity.duration().into(),
            is_active: entity.is_active().into()
        }
    }
}
