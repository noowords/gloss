use domain::aggregates::specialist::{ Specialist };

use crate::persistence::mysql::features::users::rows::value_objects::{ MySqlUserIdRow };

#[derive(Debug, Clone, sqlx::FromRow)]
pub struct MySqlSpecialistRow {
    pub user_id: MySqlUserIdRow
}

impl From<MySqlSpecialistRow> for Specialist {
    fn from(row: MySqlSpecialistRow) -> Self {
        Self::restore(
            row.user_id.into()
        )
    }
}

impl From<&Specialist> for MySqlSpecialistRow {
    fn from(entity: &Specialist) -> Self {
        Self {
            user_id: entity.user_id().into()
        }
    }
}
