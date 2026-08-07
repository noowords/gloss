use domain::aggregates::specialist::{ Specialist };

use crate::persistence::mysql::features::users::rows::value_objects::{ MySqlUserIdRow };

#[derive(Debug, Clone, sqlx::FromRow)]
pub struct MySqlSpecialistRow {
    pub user_id: MySqlUserIdRow
}

impl From<MySqlSpecialistRow> for Specialist {
    fn from(model: MySqlSpecialistRow) -> Self {
        Self::restore(
            model.user_id.into()
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
