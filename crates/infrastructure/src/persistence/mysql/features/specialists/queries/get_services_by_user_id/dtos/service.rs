use application::features::specialists::queries::get_services_by_user_id::dtos::{ Service };

use crate::persistence::mysql::features::{
    users::rows::value_objects::{ MySqlUserIdRow },
    services::rows::value_objects::{ MySqlServiceIdRow, MySqlServiceNameRow, MySqlServicePriceRow, MySqlServiceDurationRow, MySqlServiceIsActiveRow }
};

#[derive(Debug, Clone, sqlx::FromRow)]
pub struct MySqlServiceRow {
    pub id: MySqlUserIdRow,
    pub specialist_id: MySqlServiceIdRow,
    pub name: MySqlServiceNameRow,
    pub price: MySqlServicePriceRow,
    pub duration: MySqlServiceDurationRow,
    pub is_active: MySqlServiceIsActiveRow
}

impl From<MySqlServiceRow> for Service {
    fn from(row: MySqlServiceRow) -> Self {
        Self {
            id: row.id.into(),
            specialist_id: row.specialist_id.into(),
            name: row.name.into(),
            price: row.price.into(),
            duration: row.duration.into(),
            is_active: row.is_active.into()
        }
    }
}

impl From<&Service> for MySqlServiceRow {
    fn from(entity: &Service) -> Self {
        Self {
            id: entity.id.into(),
            specialist_id: entity.specialist_id.into(),
            name: entity.name.clone().into(),
            price: entity.price.clone().into(),
            duration: entity.duration.into(),
            is_active: entity.is_active.into()
        }
    }
}
