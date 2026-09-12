use application::features::account::queries::get::dtos::{ User };

use crate::persistence::mysql::features::users::rows::value_objects::{ MySqlUserIdRow, MySqlUserRoleRow };

#[derive(Debug, Clone, sqlx::FromRow)]
pub struct MySqlUserRow {
    pub id: MySqlUserIdRow,
    pub role: MySqlUserRoleRow
}

impl From<MySqlUserRow> for User {
    fn from(row: MySqlUserRow) -> Self {
        Self {
            id: row.id.into(),
            role: row.role.into()
        }
    }
}

impl From<&User> for MySqlUserRow {
    fn from(entity: &User) -> Self {
        Self {
            id: entity.id.into(),
            role: entity.role.clone().into()
        }
    }
}
