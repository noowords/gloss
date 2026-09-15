use domain::aggregates::specialists::specialist_service::{ SpecialistService };

use super::value_objects::{ MySqlSpecialistServiceSalonIdRow, MySqlSpecialistServiceServiceIdRow, MySqlSpecialistServiceSpecialistIdRow };

#[derive(Debug, Clone, sqlx::FromRow)]
pub struct MySqlSpecialistServiceRow {
    pub specialist_id: MySqlSpecialistServiceSpecialistIdRow,
    pub salon_id: MySqlSpecialistServiceSalonIdRow,
    pub service_id: MySqlSpecialistServiceServiceIdRow
}

impl TryFrom<MySqlSpecialistServiceRow> for SpecialistService {
    type Error = anyhow::Error;
    
    fn try_from(row: MySqlSpecialistServiceRow) -> Result<Self, Self::Error> {
        Self::restore(uuid::Uuid::from(row.specialist_id).into(), uuid::Uuid::from(row.salon_id).into(), uuid::Uuid::from(row.service_id).into())
    }
}

impl From<&SpecialistService> for MySqlSpecialistServiceRow {
    fn from(entity: &SpecialistService) -> Self {
        Self {
            specialist_id: uuid::Uuid::from(entity.specialist_id()).into(),
            salon_id: uuid::Uuid::from(entity.salon_id()).into(),
            service_id: uuid::Uuid::from(entity.service_id()).into()
        }
    }
}
