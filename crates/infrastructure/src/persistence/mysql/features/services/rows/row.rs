use domain::aggregates::service::{ Service };

use crate::persistence::mysql::features::users::rows::value_objects::{ MySqlUserIdRow };

use super::value_objects::{ MySqlServiceIdRow, MySqlServiceNameRow, MySqlServicePriceRow, MySqlServiceDurationRow, MySqlServiceIsActiveRow };

#[derive(Debug, Clone, sqlx::FromRow)]
pub struct MySqlServiceRow {
    pub id: MySqlServiceIdRow,
    pub specialist_id: MySqlUserIdRow,
    pub name: MySqlServiceNameRow,
    pub price: MySqlServicePriceRow,
    pub duration: MySqlServiceDurationRow,
    pub is_active: MySqlServiceIsActiveRow
}

impl TryFrom<MySqlServiceRow> for Service {
    type Error = anyhow::Error;
    
    fn try_from(model: MySqlServiceRow) -> Result<Self, Self::Error> {
        Ok(Self::restore(
            model.id.into(),
            model.specialist_id.into(),
            model.name.into(),
            model.price.try_into()?,
            model.duration.into(),
            model.is_active.into()
        ))
    }
}

impl From<&Service> for MySqlServiceRow {
    fn from(entity: &Service) -> Self {
        Self {
            id: entity.id().into(),
            specialist_id: entity.specialist_id().into(),
            name: entity.name().into(),
            price: entity.price().into(),
            duration: entity.duration().into(),
            is_active: entity.is_active().into()
        }
    }
}
