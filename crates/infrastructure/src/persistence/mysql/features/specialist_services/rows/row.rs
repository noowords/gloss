use domain::aggregates::specialist_service::{ SpecialistService };

use crate::persistence::mysql::features::{
    users::rows::value_objects::{ MySqlUserIdRow },
    services::rows::value_objects::{ MySqlServiceIdRow }
};

use super::value_objects::{ MySqlSpecialistServiceIsActiveRow };

#[derive(Debug, Clone, sqlx::FromRow)]
pub struct MySqlSpecialistServiceRow {
    pub specialist_id: MySqlUserIdRow,
    pub service_id: MySqlServiceIdRow,
    pub is_active: MySqlSpecialistServiceIsActiveRow
}

impl TryFrom<MySqlSpecialistServiceRow> for SpecialistService {
    type Error = anyhow::Error;
    
    fn try_from(row: MySqlSpecialistServiceRow) -> Result<Self, Self::Error> {
        Ok(Self::restore(
            row.specialist_id.into(),
            row.service_id.into(),
            row.is_active.into()
        ))
    }
}

impl From<&SpecialistService> for MySqlSpecialistServiceRow {
    fn from(entity: &SpecialistService) -> Self {
        Self {
            specialist_id: entity.specialist_id().into(),
            service_id: entity.service_id().into(),
            is_active: entity.is_active().into()
        }
    }
}
